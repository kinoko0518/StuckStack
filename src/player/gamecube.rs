use std::f32::consts::PI;

use bevy::prelude::*;

use crate::player::{GameCube, Player};

#[derive(Component)]
pub struct Attack {
    progress: f32,
}

const ATTACK_DURATION: f32 = 0.3;

pub fn attack_input(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    gc_query: Query<Entity, (With<GameCube>, Without<Attack>)>,
) {
    if input.just_pressed(MouseButton::Right) {
        for gc in gc_query {
            commands.entity(gc).insert(Attack { progress: 0. });
        }
    }
}

pub fn attack(
    mut commands: Commands,
    query: Query<(&mut Transform, &mut Attack, Entity), With<GameCube>>,
    q_player: Query<&Player>,
    time: Res<Time>,
) {
    let player = q_player.single().unwrap();
    for (mut transform, mut attack, entity) in query {
        const ATTACK_LENGTH: f32 = 50.;
        transform.translation = Vec3::new(player.angle.cos(), player.angle.sin(), 0.)
            * attack.progress.sin()
            * ATTACK_LENGTH;
        attack.progress += time.delta_secs() / ATTACK_DURATION;
        if attack.progress > PI {
            commands.entity(entity).remove::<Attack>();
        }
    }
}
