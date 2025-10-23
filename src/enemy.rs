mod follow_player;
mod player_detection;

use bevy::prelude::*;

use crate::enemy::{
    follow_player::{FollowPlayer, follow_closest_player},
    player_detection::{PlayerDetectPlugin, Vision},
};

#[derive(Component)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyResource>()
            .add_systems(Startup, init_resource)
            .add_systems(PostStartup, spawn_enemy)
            .add_systems(Update, follow_closest_player)
            .add_plugins(PlayerDetectPlugin);
    }
}

pub fn spawn_enemy(mut commands: Commands, enemy_res: Res<EnemyResource>) {
    for _ in 0..4 {
        spawn_almican(&mut commands, &enemy_res);
    }
}

#[derive(Resource, Default)]
pub struct EnemyResource {
    almican: Handle<Image>,
}

pub fn init_resource(mut res: ResMut<EnemyResource>, asset_server: Res<AssetServer>) {
    res.almican = asset_server.load("embedded://images/enemy/almican.png");
}

#[derive(Component)]
struct Enemy {
    pub size_radius: f32,
    pub found_player: Vec<Entity>,
}

fn spawn_almican(commands: &mut Commands, enemy_res: &Res<EnemyResource>) {
    commands.spawn((
        // Enemy {
        //     size_radius: 64.,
        //     found_player: Vec::new(),
        // },
        // Vision { vision: 320. },
        // FollowPlayer { speed: 80. },
        Sprite::from_image(enemy_res.almican.clone()),
        Transform::from_xyz(0., 0., 0.),
    ));
}
