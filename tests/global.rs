mod utils;
use std::f32::consts::TAU;

use bevy::transform::TransformBundle;
use utils::*;

proptest! {
	#[test]
	fn global_ignores_rotation(rot1 in 0.0f32 .. 1.0f32, rot2 in 0.0f32 .. 1.0f32) {
		let mut app = test_app(None);

		let mut parent = app.world.spawn((
			TransformBundle::from_transform(Transform::from_rotation(Quat::from_rotation_z(TAU * rot1))),
			RigidBody::Dynamic,
			ExternalForce::ZERO.with_persistence(false),
			Collider::capsule(1.0, 1.0),
		));

		// child with internal force
		parent.with_children(|parent| {
			parent.spawn((
				TransformBundle::from_transform(Transform::from_rotation(Quat::from_rotation_x(TAU * rot2))),
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
	fn local_factors_rotation_parent(rot1 in 0.1f32 .. 0.9f32) {
		let mut app = test_app(None);

		let mut parent = app.world.spawn((
			TransformBundle::from_transform(Transform::from_rotation(Quat::from_rotation_z(TAU * rot1))),
			RigidBody::Dynamic,
			ExternalForce::ZERO.with_persistence(false),
			Collider::capsule(1.0, 1.0),
		));

		// child with internal force
		parent.with_children(|parent| {
			parent.spawn((
				TransformBundle::default(),
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
		assert!(get_parent_transform(&mut app.world).translation.x != 0.0, "Parent transform .x != 0: {:?}", get_parent_transform(&mut app.world).translation);
		assert!(get_parent_transform(&mut app.world).translation.z == 0.0);
	}

	#[test]
	fn local_factors_rotation_child(rot_child in 0.1f32 .. 0.9f32) {
		let mut app = test_app(None);

		let mut parent = app.world.spawn((
			TransformBundle::from_transform(Transform::from_rotation(Quat::from_rotation_z(0.0))),
			RigidBody::Dynamic,
			ExternalForce::ZERO.with_persistence(false),
			Collider::capsule(1.0, 1.0),
		));

		// child with internal force
		parent.with_children(|parent| {
			parent.spawn((
				TransformBundle::from_transform(Transform::from_rotation(Quat::from_rotation_z(TAU * rot_child))),
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
