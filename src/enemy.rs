mod attack;
mod movement;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    attractable::HP,
    enemy::{
        attack::{MeleeAttack, attacking_animation, melee_attack},
        movement::{FollowPlayer, follow_closest_player, leave_from_other_enemy},
    },
};

#[derive(Component)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyResource>()
            .add_systems(Startup, init_resource)
            .add_systems(PostStartup, spawn_enemy)
            .add_systems(
                Update,
                (
                    follow_closest_player,
                    leave_from_other_enemy,
                    melee_attack,
                    attacking_animation,
                ),
            );
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
}

#[derive(Component)]
struct Vision {
    vision: f32,
}

fn spawn_almican(commands: &mut Commands, enemy_res: &Res<EnemyResource>) {
    let mut rng = rand::rng();
    commands.spawn((
        Enemy { size_radius: 64. },
        Vision { vision: 320. },
        FollowPlayer { speed: 80. },
        Sprite::from_image(enemy_res.almican.clone()),
        Transform::from_xyz(
            rng.random_range((-300.)..(300.)),
            rng.random_range((-300.)..(300.)),
            0.,
        ),
        HP::new(20),
        MeleeAttack {
            max_duration: 1.,
            duration: 1.,
            range: 128.,
            attack_power: 5,
        },
    ));
}
