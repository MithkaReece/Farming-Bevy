pub mod setup_money_ui_system;
pub mod sync_money_ui_system;
pub mod setup_selected_item_ui_system;
pub mod sync_selected_item_ui_system;

use self::setup_money_ui_system::*;
use self::sync_money_ui_system::*;
use self::setup_selected_item_ui_system::*;
use self::sync_selected_item_ui_system::*;

use bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_money_ui, setup_selected_item_ui));
        app.add_systems(Update, (sync_money_ui,sync_selected_item_ui));
    }
}
