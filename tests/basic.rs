use bevy::prelude::*;
use bevy_xpbd3d_parenting::prelude::*;
pub use bevy_xpbd_3d::prelude::*;

fn main() {
    let mut app = App::new();

    // initialize plugins
    app.add_plugins((
        DefaultPlugins,
        // bevy xpbd
        bevy_xpbd_3d::prelude::PhysicsPlugins::default(),
        // custom plugin, does not technically depend on bevy_xpbd_3d
        // but it makes sense to add afterwards
        bevy_xpbd3d_parenting::PhysicsParentingPlugin::default(),
    ));

    // spawn parent (example)
    app.world
        .spawn((
            PbrBundle {
                // add your own things here
                ..default()
            },
            RigidBody::Dynamic,
            // NB: Parent must have external force that is NOT persistent!
            // Since this is not the default, add it manually
            ExternalForce::ZERO.with_persistence(false),
            // bevy_xpbd requires that [RigidBody]s have a collider
            // which is used to compute mass and center of gravity
            Collider::capsule(1.0, 1.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    // add your own things here
                    ..default()
                },
                InternalForce(Vec3::new(0., 3., 0.)),
            ));
        });
}
