mod attractable;
mod enemy;
mod player;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use crate::{attractable::AttractablePlugin, enemy::EnemyPlugin, player::PlayerPlugin};

#[derive(States, Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MacroState {
    ESCMenu,
    #[default]
    Playing,
    MainMenu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EmbeddedAssetPlugin::default())
        .init_state::<MacroState>()
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(AttractablePlugin)
        .run();
}
