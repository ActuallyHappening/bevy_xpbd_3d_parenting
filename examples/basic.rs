use bevy::prelude::*;
use bevy_xpbd3d_parenting::InternalForce;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            bevy_xpbd3d_parenting::PhysicsParentingPlugin,
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
            mesh: meshs.add(Mesh::from(shape::Plane {
                size: 10.,
                ..default()
            })),
            material: materials.add(Color::BLACK.into()),
            ..default()
        },
        RigidBody::Static,
    ));

    // cube
    let mut cube = commands.spawn((
        PbrBundle {
            mesh: meshs.add(Mesh::from(shape::Cube { size: 1. })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0., 5., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        ExternalForce::ZERO,
        // doesn't really matter what actual computed collider you chose
        // you could just add manual collider as well
        AsyncCollider(ComputedCollider::ConvexHull),
    ));

    // cube child 1: normal child
    cube.with_children(|cube| {
        cube.spawn((
            PbrBundle {
                mesh: meshs.add(Mesh::from(shape::Cube { size: 0.5 })),
                // lighter red child
                material: materials.add(Color::rgb(0.5, 0.0, 0.0).into()),
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
                mesh: meshs.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    ..default()
                })),
								// blue child
                material: materials.add(Color::BLUE.into()),
                ..default()
            },
            // no rigidbody
            // no external force
            // for this specific, using internal force
            AsyncCollider(ComputedCollider::ConvexHull),
            InternalForce(Vec3::new(0., -10., 0.)),
        ));
    });
}
