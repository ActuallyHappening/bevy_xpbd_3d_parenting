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

fn get<T: Component + Clone>(e: Entity) -> impl Fn(&mut World) -> T {
	move |world| world.entity(e).get::<T>().unwrap().clone()
}
fn set<T: Component + Clone>(e: Entity) -> impl Fn(&mut World, T) {
	move |world, value| {
		world.entity_mut(e).insert(value.clone());
	}
}

/// This library depends heavily on other libraries,
/// which require a few frames each to setup.
const SETUP_ITERATIONS: u8 = 4;

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
	let get_parent_transform = get::<Transform>(parent);
	let get_parent_height = |world: &mut World| get_parent_transform(world).translation.y;

	assert_eq!(get_parent_height(&mut app.world), starting_y);

	for _ in 0..SETUP_ITERATIONS {
		app.update();
	}

	assert!(get_parent_height(&mut app.world) > starting_y);
}

#[test]
fn assert_external_forces_clear() {
	let mut app = test_app("info");

	let force = Vec3::new(0.0, 0.0, 100.0);
	let parent = app.world.spawn((
		TransformBundle::default(),
		RigidBody::Dynamic,
		ExternalForce::new(force).with_persistence(false),
		Collider::capsule(1.0, 1.0),
	)).id();

	let get = get::<ExternalForce>(parent);
	let set = set::<ExternalForce>(parent);

	for _ in 0..SETUP_ITERATIONS {
		app.update();
	}

	for i in 0..10 {
		let current = get(&mut app.world).force();
		#[cfg(feature = "debug")]
		println!("i: {i}, current: {:?}", current);
		assert_eq!(current, Vec3::ZERO);
		set(&mut app.world, ExternalForce::new(force + Vec3::X * i as f32).with_persistence(false));
		app.update();
	}
}

#[test]
fn invariant_constant_external_force() {
	let mut app = test_app("info");

	// let internal_force: Vec3 = Vec3::new(random(), random(), random());
	let internal_force: Vec3 = Vec3::new(0.0, 0.0, 100.0);

	// spawn parent
	let mut parent = app.world.spawn((
		TransformBundle::default(),
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
			InternalForce(internal_force),
		));
	});

	let parent = parent.id();
	let get_parent_external_force = get::<ExternalForce>(parent);

	assert_eq!(
		get_parent_external_force(&mut app.world).force(),
		Vec3::ZERO
	);

	for _ in 0..SETUP_ITERATIONS {
		app.update();
	}

	for _ in 0..10 {
		assert_eq!(
			get_parent_external_force(&mut app.world).force(),
			internal_force,
		);
		#[cfg(feature = "debug")]
		info!(
			"Force: {:?}",
			get_parent_external_force(&mut app.world).force()
		);
		app.update();
	}
}
