use bevy::prelude::*;

use crate::{
    enemy::{Enemy, Vision},
    player::Player,
};

#[derive(Component)]
pub struct FollowPlayer {
    pub speed: f32,
}

pub fn follow_closest_player(
    time: Res<Time>,
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<(&mut Transform, &FollowPlayer, &Enemy, &Vision)>,
    )>,
) {
    let players = queries
        .p0()
        .iter()
        .map(|trans| trans.translation)
        .collect::<Vec<Vec3>>();
    for (mut transform, follow_player, enemy, vision) in queries.p1() {
        let players = players.clone();
        let mut players = players
            .into_iter()
            .filter(|trans| trans.distance(transform.translation) < vision.vision)
            .collect::<Vec<Vec3>>();
        players.sort_by(|vec1, vec2| {
            vec2.distance(transform.translation)
                .total_cmp(&vec1.distance(transform.translation))
        });
        if let Some(closest) = players.last() {
            let difference: Vec3 = closest - transform.translation;
            if difference.length() > enemy.size_radius {
                transform.translation +=
                    difference.normalize_or_zero() * follow_player.speed * time.delta_secs();
            }
        }
    }
}

const LEAVE_POWER: f32 = 40.;
pub fn leave_from_other_enemy(enemies: Query<(&mut Transform, &Enemy, Entity)>, time: Res<Time>) {
    let enemy_pos = enemies
        .iter()
        .map(|(transform, _, entity)| (transform.translation, entity))
        .collect::<Vec<(Vec3, Entity)>>();
    for (mut transform, enemy, own_entity) in enemies {
        let in_collision = enemy_pos
            .iter()
            .filter(|(_, entity)| own_entity != *entity)
            .filter(|(vec3, _)| transform.translation.distance(*vec3) < enemy.size_radius)
            .map(|(vec3, _)| vec3 - transform.translation);
        let sum: Vec3 = in_collision.clone().sum::<Vec3>();
        let length = in_collision.count();
        transform.translation +=
            (-(sum / (length as f32))).normalize_or_zero() * time.delta_secs() * LEAVE_POWER;
    }
}
