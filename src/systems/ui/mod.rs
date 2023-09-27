pub mod setup_ui_system;

use self::setup_ui_system::*;

use bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
      app.add_systems(Startup, setup_ui);
    }
}
