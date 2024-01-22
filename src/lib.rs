#![doc = include_str!("../README.md")]
//! Implementation Details

#![allow(clippy::type_complexity)]
use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use serde::{Deserialize, Serialize};

pub mod prelude {
	pub use crate::{InternalForce, ParentingPlugin};
	pub(crate) use bevy::prelude::*;
	pub(crate) use bevy_xpbd_3d::prelude::*;
}

#[derive(Debug)]
pub struct ParentingPlugin {
	/// Holds a label/reference to the schedule that [bevy_xpbd_3d] is running on.
	/// This allows for properly scheduling systems correctly, 'undefined' behavior
	/// occurs if this is set to a different schedule than [bevy_xpbd_3d::PhysicsPlugins] is running on.
	bevy_xpbd_schedule: InternedScheduleLabel,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ParentingSystemSet {
	ManuallyClearForces,
	PropagateInternalForces,
}

impl ParentingPlugin {
	/// Creates a [ParentingPlugin], passing in *the same schedule you are running [bevy_xpbd_3d] on*.
	/// E.g.
	/// ```rust
	/// use bevy::prelude::*;
	/// # let mut app = App::new();
	///
	/// let physics_schedule = Update; // or FixedUpdate, see bevy_xpbd docs
	/// app.add_plugins((
	///   MinimalPlugins,
	///   bevy_xpbd_3d::prelude::PhysicsPlugins::new(physics_schedule.clone()),
	///   bevy_xpbd_3d_parenting::prelude::ParentingPlugin::new(physics_schedule),
	/// ));
	/// ````
	pub fn new(bevy_xpbd_schedule: impl ScheduleLabel) -> Self {
		Self {
			bevy_xpbd_schedule: bevy_xpbd_schedule.intern(),
		}
	}
}

impl Plugin for ParentingPlugin {
	fn build(&self, app: &mut App) {
		#[allow(clippy::upper_case_acronyms)]
		type PSS = ParentingSystemSet;

		app
			.add_systems(
				self.bevy_xpbd_schedule,
				(
					Self::manually_clear_forces.in_set(PSS::ManuallyClearForces),
					Self::propagate_internal_forces.in_set(PSS::PropagateInternalForces),
					// #[cfg(feature = "debug")]
					// helper_warnings,
				)
					.after(PhysicsSet::Prepare)
					.before(PhysicsSet::StepSimulation),
			)
			.register_type::<InternalForce>();
	}
}

/// Synced with parents
#[derive(
	Reflect, Component, Debug, Clone, Copy, Deref, DerefMut, Serialize, Deserialize, Default,
)]
#[reflect(Component)]
pub struct InternalForce(pub Vec3);

impl InternalForce {
	pub const ZERO: Self = InternalForce(Vec3::ZERO);

	pub fn inner(&self) -> Vec3 {
		self.0
	}

	pub fn get(&self) -> Vec3 {
		self.0
	}

	pub fn into_inner(self) -> Vec3 {
		self.0
	}

	pub fn set(&mut self, value: Vec3) {
		self.0 = value;
	}
}

mod systems {
	use crate::prelude::*;
	impl super::ParentingPlugin {
		/// Mutates parent's [`ExternalForce`] component depending on it's
		/// children that are not [`RigidBody`]'s but have an [`InternalForce`] component.
		///
		/// This is automatically scheduled in [ParentingPlugin] but is public so
		/// that end users can manually schedule this whenever they want.
		pub(super) fn propagate_internal_forces(
			mut parents: Query<(&mut ExternalForce, &CenterOfMass, &GlobalTransform), With<RigidBody>>,
			children: Query<
				(&Parent, &InternalForce, &GlobalTransform),
				(Without<RigidBody>, Without<ExternalForce>),
			>,
		) {
			for (collider_parent, internal_force, child_global_transform) in children.iter() {
				if let Ok((mut parents_force, center_of_mass, parent_global_transform)) =
					parents.get_mut(collider_parent.get())
				{
					if parents_force.persistent {
						warn!("A child entity (with an `InternalForce` but no `RigidBody`) is a child of a RigidBody entity with a persistent ExternalForce. \
								This is not supported, as child entities' `ExternalForce` is updated every (physics) frame by the `ParentingPlugin`");
					} else {
						let parent_child_transform =
							child_global_transform.reparented_to(parent_global_transform);

						let internal_quat = parent_child_transform.rotation;
						let internal_force = internal_quat.mul_vec3(internal_force.0);
						let internal_point = parent_child_transform.translation;

						let previous_parents_force = *parents_force;
						// #[cfg(feature = "debug")]
						// {
						// 	if previous_parents_force.force() != Vec3::ZERO {
						// 		warn!("Not reset, has changed: {}", parents_force.is_changed());
						// 	}
						// 	// assert_eq!(previous_parents_force.force(), Vec3::ZERO);
						// }

						// the meat of the whole library
						parents_force.apply_force_at_point(internal_force, internal_point, center_of_mass.0);
						parents_force.set_changed();

						#[cfg(feature = "debug")]
						debug!(
						"Applying internal force {:?} at point {:?} on existing force {:?}, resulting in {:?}",
						internal_force, internal_point, previous_parents_force, parents_force
					);
					}
				} else {
					warn!("The parent of an entity with `InternalForce` points to a non-`RigidBody` entity");
				};
			}
		}

		pub(super) fn manually_clear_forces(mut external_forces: Query<&mut ExternalForce>) {
			for mut external_force in external_forces.iter_mut() {
				if !external_force.persistent {
					#[cfg(feature = "debug")]
					trace!("Clearing external force {:?}", external_force);
					external_force.clear();
				}
			}
		}
	}
}
