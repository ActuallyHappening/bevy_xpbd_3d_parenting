//! Shows the difference between an [InternalForce::Global] and [InternalForce::Local].
//! 
//! The spinning of the structure is erratic, since one child is in global space (down) for its
//! [InternalForce] and the other is in local space (right and up).

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
			transform: Transform::from_xyz(0., 0., 0.),
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
				mesh: meshs.add(Cuboid::default()),
				// lighter red child
				material: materials.add(Color::rgb(0.5, 0.0, 0.0)),
				transform: Transform::from_xyz(3.0, 0.0, 0.0),
				..default()
			},
			Collider::capsule(0.5, 0.5),
			// ! This will pull the structure gradually rightward but not consistently since it
			// ! depends on the parents rotation
			InternalForce::new_local_forward_right_up(0.0, 0.5, 1.0),
		));
	});

	// sphere child 2: using globinternal force
	cube.with_children(|cube| {
		cube.spawn((
			PbrBundle {
				mesh: meshs.add(Sphere::default().mesh().uv(16, 18)),
				// blue child
				material: materials.add(Color::BLUE),
				// to the left a bit
				transform: Transform::from_xyz(-3.0, 0.0, 0.0),
				..default()
			},
			AsyncCollider(ComputedCollider::ConvexHull),
			// ! This pushes downwards in global space, meaning it will pull the whole
			// ! structure downward regardless of local rotation.
			InternalForce::new_global(Vec3::new(0., -1.0, 0.0)),
		));
	});
}
