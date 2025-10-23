use bevy::prelude::*;

use crate::{enemy::Enemy, player::Player};

#[derive(Component)]
pub struct FollowPlayer {
    pub speed: f32,
}

pub fn follow_closest_player(
    mut queries: ParamSet<(
        Query<(&Transform, Entity), With<Player>>,
        Query<(&mut Transform, &Enemy, &FollowPlayer)>,
    )>,
) {
    let players = queries
        .p0()
        .iter()
        .map(|(trans, entity)| (trans.translation, entity))
        .collect::<Vec<(Vec3, Entity)>>();
    for (mut transform, enemy, follow_player) in queries.p1() {
        let mut players = players
            .iter()
            .filter(|(_, pl_entity)| enemy.found_player.contains(pl_entity))
            .map(|(trans, _)| trans)
            .collect::<Vec<&Vec3>>();
        players.sort_by(|vec1, vec2| {
            vec2.distance(transform.translation)
                .total_cmp(&vec1.distance(transform.translation))
        });
        if let Some(closest) = players.last() {
            let difference: Vec3 = *closest - transform.translation;
            transform.translation += difference.normalize() * follow_player.speed;
        }
    }
}
