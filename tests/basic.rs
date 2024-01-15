use bevy::{
	log::{Level, LogPlugin},
	prelude::*,
};
pub use bevy_xpbd_3d::prelude::*;
use bevy_xpbd_3d_parenting::prelude::*;
use rand::random;

fn test_app(log_level: &str) -> App {
	let mut app = App::new();
	// initialize plugins
	app.add_plugins((
		MinimalPlugins,
		LogPlugin {
			// basic is name of integration test
			filter: format!(
				"basic={log_level},bevy_xpbd_3d_parenting={log_level}",
				log_level = log_level
			),
			level: Level::INFO,
		},
		bevy_xpbd_3d::prelude::PhysicsPlugins::new(Update),
		bevy_xpbd_3d_parenting::ParentingPlugin::new(Update),
	));

	app
}

#[test]
fn assert_moves_up() {
	let mut app = test_app("info");

	let starting_y = random::<f32>() * 100.;

	// spawn parent
	let mut parent = app.world.spawn((
		TransformBundle::from_transform(Transform::from_translation(Vec3::Y * starting_y)),
		RigidBody::Dynamic,
		// NB: Parent must have external force that is NOT persistent!
		ExternalForce::ZERO.with_persistence(false),
		Collider::capsule(1.0, 1.0),
	));
	// spawn child
	parent.with_children(|parent| {
		parent.spawn((
			TransformBundle::default(),
			Collider::capsule(1.0, 1.0),
			InternalForce(Vec3::new(0., 300.0, 0.)),
		));
	});

	let parent = parent.id();
	let get_parent_height = |world: &mut World| {
		world.entity(parent).get::<Transform>().unwrap().translation.y
	};

	assert_eq!(get_parent_height(&mut app.world), starting_y);

	for _ in 0..3 {
		app.update();
	}

	assert!(get_parent_height(&mut app.world) > starting_y);
}

#[test]
fn invariant_constant_external_force() {
	let mut app = test_app("info");
	
}