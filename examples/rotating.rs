//! Shows a basic usage of [InternalForce].
//! This spawns a center red cube, and two children on either side.
//!
//! The child on the right has an internal force facing downwards, which should rotate the parent clockwise.
//!
//! Note also that this system is not balanced as is clearly seen, so center of mass is taken into account

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy_xpbd_3d_parenting::InternalForce;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins,
			PhysicsPlugins::new(Update),
			bevy_xpbd_3d_parenting::prelude::ParentingPlugin::new(Update),
		))
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshs: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// camera
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0., 0., 10.),
		..default()
	});

	// ground
	commands.spawn((
		PbrBundle {
			mesh: meshs.add(Plane3d::default()),
			material: materials.add(Color::BLACK),
			..default()
		},
		RigidBody::Static,
	));
	// also, remove gravity
	commands.insert_resource(Gravity(Vec3::ZERO));

	// cube
	let mut cube = commands.spawn((
		PbrBundle {
			mesh: meshs.add(Cuboid::default()),
			material: materials.add(Color::RED),
			transform: Transform::from_xyz(0., 5., 0.),
			..default()
		},
		RigidBody::Dynamic,
		// IMPORTANT: parent's external force must be non-persistent
		// so that each frame this library can update it just after it resets
		ExternalForce::ZERO.with_persistence(false),
		// doesn't really matter what actual computed collider you chose
		// you could just add manual collider as well
		AsyncCollider(ComputedCollider::ConvexHull),
	));

	// cube child 1: normal child
	cube.with_children(|cube| {
		cube.spawn((
			PbrBundle {
				// mesh: meshs.add(Mesh::from(shape::Cube { size: 0.5 })),
				mesh: meshs.add(Cuboid::default()),
				// lighter red child
				material: materials.add(Color::rgb(0.5, 0.0, 0.0)),
				transform: Transform::from_xyz(-3.0, 0.0, 0.0),
				..default()
			},
			// no rigidbody
			// no external force
			// for this specific, child no internal force
			Collider::capsule(0.5, 0.5),
		));
	});

	// sphere child 2: using internal force
	cube.with_children(|cube| {
		cube.spawn((
			PbrBundle {
				mesh: meshs.add(Sphere::default().mesh().uv(16, 18)),
				// blue child
				material: materials.add(Color::BLUE),
				// to the right a bit
				transform: Transform::from_xyz(3.0, 0.0, 0.0),
				..default()
			},
			// no rigidbody
			// no external force
			// this collider is here for demonstration of center of mass,
			// children do not need colliders
			AsyncCollider(ComputedCollider::ConvexHull),
			// ! internal force pushes downwards, which should rotate clockwise
			InternalForce::new_local_forward_right_up(0., 0., -3.0),
		));
	});
}
