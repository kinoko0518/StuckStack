mod gamecube;
mod movement;

use bevy::prelude::*;

use crate::{MacroState, attractable::HP};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerResource>()
            .add_systems(Startup, resource_init)
            .add_systems(PostStartup, player_spawn)
            .add_systems(
                Update,
                (
                    movement::movement,
                    movement::get_angle_to_mouse,
                    gamecube::attack_input,
                    gamecube::attack,
                )
                    .run_if(in_state(MacroState::Playing)),
            );
    }
}

#[derive(Resource, Default)]
pub struct PlayerResource {
    player: Handle<Image>,
    gc: Handle<Image>,
}

#[derive(Component)]
pub struct Player {
    angle: f32,
}

#[derive(Component)]
struct GameCube;

pub fn resource_init(mut player_res: ResMut<PlayerResource>, asset_server: Res<AssetServer>) {
    player_res.player = asset_server.load("embedded://images/player/placeholder_player.png");
    player_res.gc = asset_server.load("embedded://images/player/gc.png");
}

pub fn player_spawn(mut commands: Commands, pl_res: Res<PlayerResource>) {
    commands
        .spawn((
            Player { angle: 0. },
            Transform::from_xyz(0., 0., 0.),
            Sprite::from_image(pl_res.player.clone()),
            HP::new(100),
        ))
        .with_child((
            GameCube,
            Transform::from_xyz(0., 0., 0.),
            Sprite::from_image(pl_res.gc.clone()),
        ))
        .with_child(Camera2d);
}
