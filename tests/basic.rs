use bevy::{
	log::{Level, LogPlugin},
	prelude::*,
};
use bevy_xpbd3d_parenting::prelude::*;
pub use bevy_xpbd_3d::prelude::*;

#[test]
fn assert_moves_up() {
	let mut app = App::new();

	// initialize plugins
	app.add_plugins((
		MinimalPlugins,
		LogPlugin {
			// basic is name of integration test
			filter: "basic=trace,bevy_xpbd3d_parenting=trace".into(),
			level: Level::INFO,
		},
		// bevy xpbd
		bevy_xpbd_3d::prelude::PhysicsPlugins::new(Update),
		// custom plugin, does not technically depend on bevy_xpbd_3d
		// but it makes sense to add afterwards.
		// Also, the schedule must be the same as the one used for bevy_xpbd_3d
		bevy_xpbd3d_parenting::ParentingPlugin::new(Update),
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
	parent.with_children(|parent| {
		parent.spawn((
			PbrBundle {
				// add your own things here
				..default()
			},
			// rather high because this only applies for one frame
			InternalForce(Vec3::new(0., 300.0, 0.)),
		));
	});

	let parent = parent.id();
	// let child;

	{
		let parent_transform = app.world.get::<Transform>(parent).unwrap();
		trace!("Parent transform before physics: {:?}", parent_transform);
		assert!(parent_transform.translation.y == 0.0);

		let parent_children = app.world.get::<Children>(parent).unwrap();
		trace!("Parent children before physics: {:?}", parent_children);
		assert!(parent_children.len() == 1);
		// child = parent_children[0];
	}

	app.update();

	{
		let parent_transform = app.world.get::<Transform>(parent).unwrap();
		debug!("Parent transform after physics: {:?}", parent_transform);
		assert!(parent_transform.translation.y > 0.0);
	}
}
