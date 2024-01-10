use bevy::{prelude::*, render::RenderPlugin, winit::WinitPlugin};
use bevy_xpbd3d_parenting::prelude::*;
pub use bevy_xpbd_3d::prelude::*;
use test_log::test;

#[test]
fn assert_moves() {
	let mut app = App::new();

	// initialize plugins
	app.add_plugins((
		MinimalPlugins,
		// bevy xpbd
		bevy_xpbd_3d::prelude::PhysicsPlugins::new(Update),
		// custom plugin, does not technically depend on bevy_xpbd_3d
		// but it makes sense to add afterwards
		bevy_xpbd3d_parenting::PhysicsParentingPlugin::default(),
	));

	// spawn parent (example)
	let mut parent = app.world.spawn((
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
	));
	let mut child = None;
	parent.with_children(|parent| {
		child = Some(
			parent
				.spawn((
					PbrBundle {
						// add your own things here
						..default()
					},
					InternalForce(Vec3::new(0., 3., 0.)),
				))
				.id(),
		);
	});
	let parent = parent.id();

	app.world.run_schedule(Main);

	assert!(child.is_some());

	assert!(app.world.get::<Transform>(parent).unwrap().translation.y > 0.0);
}
