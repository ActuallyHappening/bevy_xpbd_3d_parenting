#![allow(clippy::type_complexity)]
#[doc = include_str!("../README.md")]

use bevy::prelude::*;
use bevy_xpbd_3d::{plugins::integrator::clear_forces_and_impulses, prelude::*, PhysicsSchedule};
use serde::{Deserialize, Serialize};

pub mod prelude {
	// pub use bevy_xpbd_3d::prelude::*;
	pub use crate::{PhysicsParentingPlugin, InternalForce};
}

#[derive(Debug, Default)]
pub struct PhysicsParentingPlugin {}

impl Plugin for PhysicsParentingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PhysicsSchedule,
            (apply_internal_forces, #[cfg(feature = "debug")] helper_warnings)
                .after(bevy_xpbd_3d::PhysicsStepSet::SpatialQuery)
                .after(clear_forces_and_impulses),
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

    pub fn set(&mut self, value: Vec3) {
        self.0 = value;
    }
}

/// Mutates parent's [`ExternalForce`] component depending on it's
/// children that are not [`RigidBody`]'s but have an [`InternalForce`] component
fn apply_internal_forces(
    mut parents: Query<(&mut ExternalForce, &CenterOfMass, &GlobalTransform), With<RigidBody>>,
    children: Query<
        (&ColliderParent, &InternalForce, &GlobalTransform),
        (Without<RigidBody>, Without<ExternalForce>),
    >,
) {
    for (collider_parent, internal_force, child_global_transform) in children.iter() {
        if let Ok((mut parents_force, center_of_mass, parent_global_transform)) =
            parents.get_mut(collider_parent.get())
        {
            if parents_force.persistent {
                warn!("A child entity (with an InternalForce but no RigidBody) is a (ColliderParent) parent of a RigidBody entity with a persistent ExternalForce. \
								This is not supported, as child entities in this format continuously update their parent's ExternalForce component, therefor making the parent's ExternalForce not persistent!");
            } else {
                let parent_child_transform =
                    child_global_transform.reparented_to(parent_global_transform);

                if parent_child_transform.scale.round() != Vec3::splat(1.) {
                    warn!("Scaling is not yet supported for `InternalForce` components. PRs welcome! Offending transform: {:?}", parent_child_transform);
                }

                let internal_quat = parent_child_transform.rotation;
                let internal_force = internal_quat.mul_vec3(internal_force.0);
                let internal_point = parent_child_transform.translation;

                parents_force.apply_force_at_point(
                    internal_force,
                    internal_point,
                    center_of_mass.0,
                );
            }
        } else {
            warn!("Collider parent points to a non-RigidBody entity");
        };
    }
}

#[cfg(feature = "debug")]
fn helper_warnings(
    possible_children: Query<
        (Has<ColliderParent>, Has<RigidBody>, Has<ExternalForce>),
        With<InternalForce>,
    >,
) {
    for (collider_parent, rigid_body, external_force) in possible_children.iter() {
        match (collider_parent, rigid_body, external_force) {
			(_, _, true) => warn!("Adding [InternalForce] on the same entity as an [ExternalForce] component will be ignored"),
			(false, true, _) => warn!("Adding [InternalForce] component to an entity with [RigidBody] will be ignored."),
			(true, true, _) => warn!("Adding [InternalForce] component to an entity that is a Rigid body and \
			 a child of another rigid body (i.e. has the [ColliderParent] component) will be ignored"),
			 (true, false, _) => { /* This is actually what we want */},
			 (false, false, _) => warn!("Adding [InternalForce] component to an entity that is not a child of a [RigidBody] is \
			 pointless, as [InternalForce] applied forces only to parents"),
		}
    }
}
