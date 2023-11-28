use bevy::prelude::*;
use bevy_xpbd_3d::{
	plugins::integrator::clear_forces_and_impulses, prelude::*, PhysicsSchedule,
};
use serde::{Deserialize, Serialize};

pub struct PhysicsParentingPlugin;

impl Plugin for PhysicsParentingPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(
				PhysicsSchedule,
				apply_internal_forces
					.after(bevy_xpbd_3d::PhysicsStepSet::SpatialQuery)
					.after(clear_forces_and_impulses),
			)
			.register_type::<InternalForce>();
	}
}

/// Synced with parents
#[derive(Reflect, Component, Debug, Clone, Copy, Deref, DerefMut, Serialize, Deserialize, Default)]
#[reflect(Component)]
pub struct InternalForce(pub Vec3);

impl InternalForce {
	pub const ZERO: Self = InternalForce(Vec3::ZERO);

	pub fn inner(&self) -> Vec3 {
		self.0
	}

	pub fn set(&mut self, value: Vec3) {
		self.0 = value;
	}
}

/// Mutates parent's [`ExternalForce`] component depending on it's
/// children that are not [`RigidBody`]'s but have an [`InternalForce`] component
fn apply_internal_forces(
	mut parents: Query<(&mut ExternalForce, &CenterOfMass, &GlobalTransform), With<RigidBody>>,
	children: Query<(&ColliderParent, &InternalForce, &GlobalTransform), Without<RigidBody>>,
) {
	for (collider_parent, internal_force, child_global_transform) in children.iter() {
		if let Ok((mut parents_force, center_of_mass, parent_global_transform)) =
			parents.get_mut(collider_parent.get())
		{
			if parents_force.persistent {
				warn!("A child entity (with an InternalForce but no RigidBody) is a (ColliderParent) parent of a RigidBody entity with a persistent ExternalForce. \
								This is not supported, as child entities in this format continuously update their parent's ExternalForce component, therefor making the parent's ExternalForce not persistent!");
			} else {
				let parent_child_transform = child_global_transform.reparented_to(parent_global_transform);

				if parent_child_transform.scale.round() != Vec3::splat(1.) {
					warn!("Scaling is not yet supported for `InternalForce` components. PRs welcome! Offending transform: {:?}", parent_child_transform);
				}

				let internal_quat = parent_child_transform.rotation;
				let internal_force = internal_quat.mul_vec3(internal_force.0);
				let internal_point = parent_child_transform.translation;

				parents_force.apply_force_at_point(internal_force, internal_point, center_of_mass.inner());
			}
		} else {
			warn!("Collider parent points to a non-RigidBody entity");
		};
	}
}
