# bevy_xpbd_3d_parenting
Allows children of a `bevy_xpbd_3d` `RigidBody` to exert forces on their parents.

## Installation
```toml
# Use the latest release of bevy_xpbd_3d_parenting
[dependencies.bevy_xpbd_3d_parenting]
version = "0.1"
default-features = false
```

## Quick usage example:
See the (examples)[./examples] for complete examples.
```rust,no_run
// Shows a basic usage of [InternalForce].

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
	// cube
	let mut cube = commands.spawn((
		PbrBundle {
			mesh: meshs.add(Mesh::from(shape::Cube { size: 1. })),
			transform: Transform::from_xyz(0., 5., 0.),
			..default()
		},
		RigidBody::Dynamic,
		// IMPORTANT: parent's external force must be non-persistent
		// so that each frame this library can update it
		ExternalForce::ZERO.with_persistence(false),
		// Exact collider is arbitrary
		Collider::capsule(1.0, 1.0),
	));

	// sphere child: using internal force
	cube.with_children(|cube| {
		cube.spawn((
			PbrBundle {
				mesh: meshs.add(Mesh::from(shape::UVSphere {
					radius: 0.5,
					..default()
				})),
				// to the right a bit
				transform: Transform::from_xyz(3.0, 0.0, 0.0),
				..default()
			},
			// no rigidbody
			// no external force
			// internal force pushes downwards, which should rotate clockwise
			InternalForce(Vec3::new(0., -3., 0.)),
		));
	});
}
```

## Running examples
Run:
```sh
cargo r --example rotating --features bevy_xpbd_3d/async-collider
```

## Compatibility table
| Bevy | Bevy XPBD | Bevy XPBD 3D Parenting |
| ---- | --------- | ---------------------- |
| 0.12 | 0.3.3     | 0.1.0									|