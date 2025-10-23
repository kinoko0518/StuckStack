use bevy::prelude::*;

use crate::{enemy::Enemy, player::Player};

pub struct PlayerDetectPlugin;

impl Plugin for PlayerDetectPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DetectPlayer>()
            .add_systems(Update, (update_player_list, vision_player_detection))
            .add_systems(
                Update,
                (vision_player_detection).run_if(on_message::<DetectPlayer>),
            );
    }
}

#[derive(Message)]
pub struct DetectPlayer;

#[derive(Component)]
pub struct Vision {
    pub vision: f32,
}

pub fn update_player_list(mut message: MessageWriter<DetectPlayer>, query: Query<&mut Enemy>) {
    for mut enemy in query {
        enemy.found_player = Vec::new();
    }
    message.write(DetectPlayer);
}

pub fn vision_player_detection(
    mut queries: ParamSet<(
        Query<(&Transform, Entity), With<Player>>,
        Query<(&mut Enemy, &Vision, &Transform)>,
    )>,
) {
    let player_pos = queries
        .p0()
        .iter()
        .map(|(transform, entity)| (transform.translation, entity))
        .collect::<Vec<(Vec3, Entity)>>();
    for (mut enemy, vision, transform) in queries.p1() {
        let found_player: Vec<&Entity> = player_pos
            .iter()
            .filter(|(pos, _)| pos.distance(transform.translation) < vision.vision)
            .map(|(_, entity)| entity)
            .collect();
        enemy.found_player.extend(found_player);
    }
}
