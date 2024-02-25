mod utils;
use utils::*;

// proptest! {
#[ignore = "non deterministic, passes most of the time"]
#[test]
fn rotates() {
	let mut app = test_app(None);

	let mut parent = app.world.spawn((
		TransformBundle::default(),
		RigidBody::Dynamic,
		ExternalForce::ZERO.with_persistence(false),
		Collider::capsule(1.0, 1.0),
	));

	// child with internal force down right
	parent.with_children(|parent| {
		parent.spawn((
			TransformBundle::from_transform(Transform::from_xyz(5.0, 0.0, 0.0)),
			Collider::capsule(1.0, 1.0),
			// should rotate clockwise
			InternalForce::new_local(-Vec3::Y * 1000.0),
		));
	});

	let parent = parent.id();
	let get_parent_transform = get::<Transform>(parent);
	let get_parent_z_rot = |world: &mut World| {
		get_parent_transform(world)
			.rotation
			.to_euler(EulerRot::XYZ)
			.2
	};

	// hasn't rotated
	assert!(get_parent_z_rot(&mut app.world) == 0.0);

	for _ in 0..SETUP_ITERATIONS {
		app.update();
	}

	assert!(get_parent_transform(&mut app.world).translation.z == 0.0);

	assert!(
		get_parent_z_rot(&mut app.world) < 0.0,
		"Parent hasn't rotated: {:?}, transform: {:?}",
		get_parent_z_rot(&mut app.world),
		get_parent_transform(&mut app.world)
	);
}
// }
