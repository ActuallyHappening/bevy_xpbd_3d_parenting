mod utils;
use std::f32::consts::TAU;

use bevy::transform::TransformBundle;
use utils::*;

proptest! {
	#[test]
	fn global_ignores_rotation(rot1 in 0.1f32 .. 4f32, rot2 in 0.1f32 .. 4f32) {
		let mut app = test_app("info");

		let mut parent = app.world.spawn((
			TransformBundle::from_transform(Transform::from_rotation(Quat::from_rotation_z(TAU / rot1))),
			RigidBody::Dynamic,
			ExternalForce::ZERO.with_persistence(false),
			Collider::capsule(1.0, 1.0),
		));

		// child with internal force
		parent.with_children(|parent| {
			parent.spawn((
				TransformBundle::from_transform(Transform::from_rotation(Quat::from_rotation_z(TAU / rot2))),
				Collider::capsule(1.0, 1.0),
				InternalForce::new_global(-Vec3::Y * 10.0),
			));
		});

		let parent = parent.id();
		let get_parent_transform = get::<Transform>(parent);

		for _ in 0..SETUP_ITERATIONS {
			app.update();
		}

		assert!(get_parent_transform(&mut app.world).translation.y < 0.0);
		assert!(get_parent_transform(&mut app.world).translation.x == 0.0);
		assert!(get_parent_transform(&mut app.world).translation.z == 0.0);
	}

	#[test]
	fn local_factors_rotation(rot1 in 0.1f32 .. 4f32, rot2 in 0.1f32 .. 4f32) {
		let mut app = test_app("info");

		let mut parent = app.world.spawn((
			TransformBundle::from_transform(Transform::from_rotation(Quat::from_rotation_z(TAU / rot1))),
			RigidBody::Dynamic,
			ExternalForce::ZERO.with_persistence(false),
			Collider::capsule(1.0, 1.0),
		));

		// child with internal force
		parent.with_children(|parent| {
			parent.spawn((
				TransformBundle::from_transform(Transform::from_rotation(Quat::from_rotation_z(TAU / rot2))),
				Collider::capsule(1.0, 1.0),
				InternalForce::new_local(-Vec3::Y * 10.0),
			));
		});

		let parent = parent.id();
		let get_parent_transform = get::<Transform>(parent);

		for _ in 0..SETUP_ITERATIONS {
			app.update();
		}

		assert!(get_parent_transform(&mut app.world).translation.y != 0.0);
		assert!(get_parent_transform(&mut app.world).translation.x != 0.0);
		assert!(get_parent_transform(&mut app.world).translation.z == 0.0);
	}
}
