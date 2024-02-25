#![allow(dead_code)]
#![allow(unused_imports)]

use bevy::log::{Level, LogPlugin};
pub use bevy_xpbd_3d::prelude::*;
pub use bevy_xpbd_3d_parenting::prelude::*;
pub use bevy::prelude::*;
pub use proptest::proptest;

#[allow(unused_variables)]
pub fn test_app(log_level: &str) -> App {
	let mut app = App::new();
	// initialize plugins
	app.add_plugins((
		MinimalPlugins,
		// LogPlugin {
		// 	// basic is name of integration test
		// 	filter: format!(
		// 		"basic={log_level},bevy_xpbd_3d_parenting={log_level}",
		// 		log_level = log_level
		// 	),
		// 	level: Level::INFO,
		// 	update_subscriber: None,
		// },
		bevy_xpbd_3d::prelude::PhysicsPlugins::new(Update),
		bevy_xpbd_3d_parenting::ParentingPlugin::new(Update),
	));

	app
}

pub fn get<T: Component + Clone>(e: Entity) -> impl Fn(&mut World) -> T {
	move |world| world.entity(e).get::<T>().unwrap().clone()
}
pub fn set<T: Component + Clone>(e: Entity) -> impl Fn(&mut World, T) {
	move |world, value| {
		world.entity_mut(e).insert(value.clone());
	}
}

/// This library depends heavily on other libraries,
/// which require a few frames each to setup.
pub const SETUP_ITERATIONS: u8 = 5;