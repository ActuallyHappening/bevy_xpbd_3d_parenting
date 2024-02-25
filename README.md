# bevy_xpbd_3d_parenting
Allows children of a `bevy_xpbd_3d` `RigidBody` to exert forces on their parents.

## Installation
```toml
# Use the latest release of bevy_xpbd_3d_parenting
[dependencies.bevy_xpbd_3d_parenting]
version = "0.1"
default-features = false
```

## Theoretical usage
This library exports a single `Plugin`, `ParentingPlugin`, which must be added
to the app with the same `Schedule` as `bevy_xpbd_3d`'s `PhysicsPlugin`.

Parents must have:
- `RigidBody`
	- `RigidBody::Dynamic` or nothing will move
	- `Collider` so that bevy_xpbd works
- `ExternalForce` *with `persistence` set to `false`* (will warn using `tracing` if not upheld)
- `TransformBundle` for position in space:
	- `Transform`
	- `GlobalTransform`

Children must have:
- `RigidBody`, see parent
- `TransformBundle`, see parent
- **`InternalForce` to exert forces on the parent**


### Types of Internal Forces
Internal forces come in two flavours, `InternalForce::Global` and `InternalForce::Local`.
Local `InternalForce`s are in the local space of the parent, while global `InternalForce`s are in the global space.
This means local `InternalForce`s will rotate with the parent, while global `InternalForce`s will not.

Check out the [global_versus_local](./examples/global_versus_local.rs) example for a demonstration.

## Quick usage example:
See the [examples](./examples) for complete examples.
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
			mesh: meshs.add(Mesh::from(Cuboid::default())),
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
				mesh: meshs.add(Mesh::from(Sphere::new(0.5).mesh().uv(16, 32))),
				// to the right a bit
				transform: Transform::from_xyz(3.0, 0.0, 0.0),
				..default()
			},
			// no rigidbody
			// no external force
			// internal force pushes downwards, which should rotate clockwise
			InternalForce::new_local(Vec3::new(0., -3., 0.)),
		));
	});
}
```

## Running examples
Run:
```sh
cargo r --example rotating --features bevy_xpbd_3d/async-collider
cargo r --example global_versus_local --features bevy_xpbd_3d/async-collider
```

## Compatibility table
| Bevy | Bevy XPBD | Bevy XPBD 3D Parenting |
| ---- | --------- | ---------------------- |
| 0.12 | 0.3.3     | 0.1.0									|
| 0.13 | 0.4.2		 | 0.2.0									|

## Developing notes
`cargo t` for testing, which runs:
- Every combination of feature flags
- A variety of `proptest`s