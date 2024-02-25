#![doc = include_str!("../README.md")]
//! Implementation Details

#![allow(clippy::type_complexity)]
use std::ops::{Deref, DerefMut};

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
#[derive(Reflect, Component, Debug, Clone, Copy, Serialize, Deserialize)]
#[reflect(Component)]
pub enum InternalForce {
	/// A force that is applied in the global space of the parent entity.
	Global { force: Vec3, strength: f32 },
	/// A force that is applied in the local space of the parent entity,
	/// relative to the child entity.
	Local { force: Vec3, strength: f32 },
}

impl Default for InternalForce {
	fn default() -> Self {
		InternalForce::DEFAULT
	}
}

impl Deref for InternalForce {
	type Target = Vec3;

	fn deref(&self) -> &Self::Target {
		match self {
			InternalForce::Global { force, .. } => force,
			InternalForce::Local { force, .. } => force,
		}
	}
}

impl DerefMut for InternalForce {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			InternalForce::Global { force, .. } => force,
			InternalForce::Local { force, .. } => force,
		}
	}
}

impl InternalForce {
	pub const ZERO: Self = InternalForce::Local {
		force: Vec3::ZERO,
		strength: 1.0,
	};

	pub const DEFAULT: Self = Self::ZERO;

	pub const fn default() -> Self {
		Self::DEFAULT
	}

	pub fn get_strength(&self) -> f32 {
		match self {
			InternalForce::Global { strength, .. } => *strength,
			InternalForce::Local { strength, .. } => *strength,
		}
	}

	pub fn get_mut_strength(&mut self) -> &mut f32 {
		match self {
			InternalForce::Global { strength, .. } => strength,
			InternalForce::Local { strength, .. } => strength,
		}
	}

	pub fn set_strength(&mut self, strength: f32) {
		match self {
			InternalForce::Global { strength: s, .. } => *s = strength,
			InternalForce::Local { strength: s, .. } => *s = strength,
		}
	}

	pub fn with_strength(mut self, strength: f32) -> Self {
		self.set_strength(strength);
		self
	}

	/// Creates an [InternalForce] that operates in the local space of the parent entity.
	/// By default, the strength is 1.0.
	///
	/// Also see [Self::new_forward_right_up]
	pub fn new_local(force: Vec3) -> Self {
		InternalForce::Local {
			force,
			strength: 1.0,
		}
	}

	/// See [InternalForce::new_local]
	pub fn new_relative(force: Vec3) -> Self {
		Self::new_local(force)
	}

	/// Creates an [InternalForce] that operates in the global space of the parent entity.
	/// By default, the strength is 1.0.
	pub fn new_global(force: Vec3) -> Self {
		InternalForce::Global {
			force,
			strength: 1.0,
		}
	}

	/// See [InternalForce::new_global]
	pub fn new_absolute(force: Vec3) -> Self {
		Self::new_global(force)
	}

	/// Creates an [InternalForce] that operates in the local space of the parent entity,
	/// with the force being forward, right, and up. This assumes forward is in the -Z
	/// direction, right is in the +X direction, and up is in the +Y direction.
	///
	/// This is a wrapper around [Self::new_local]
	pub fn new_forward_right_up(forward: f32, right: f32, up: f32) -> Self {
		Self::new_local(Vec3::new(right, up, -forward))
	}

	/// Returns a [Vec3] representing the force, *without* the strength applied.
	/// This is naive because it may be global or local.
	/// To work out if the force is global or local, `match` on the `InternalForce`.
	pub fn get_naive_force(&self) -> Vec3 {
		**self
	}

	/// Returns a [Vec3] representing the force, *with* the strength applied.
	/// This is naive because it may be global or local.
	/// To work out if the force is global or local, `match` on the `InternalForce`.
	pub fn compute_naive_force(&self) -> Vec3 {
		self.get_naive_force() * self.get_strength()
	}

	/// Returns a non-naive [Vec3] representing the global force represented by the [InternalForce].
	/// This takes into account the strength.
	pub fn compute_global_force(&self, parent_transform: &GlobalTransform) -> Vec3 {
		match self {
			InternalForce::Global { .. } => self.compute_naive_force(),
			InternalForce::Local { .. } => parent_transform
				.compute_transform()
				.rotation
				.mul_vec3(self.compute_naive_force()),
		}
	}
}

mod systems {
	use crate::prelude::*;
	impl super::ParentingPlugin {
		/// Mutates parent's [`ExternalForce`] component depending on it's
		/// children that are not [`RigidBody`]'s but have an [`InternalForce`] component.
		/// This is automatically scheduled in [ParentingPlugin]
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
					trace!("Manually clearing external force {:?}", external_force);
					external_force.clear();
				}
			}
		}
	}
}
