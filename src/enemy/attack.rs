use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    attractable::{HP, OnHPChanged},
    player::Player,
};

#[derive(Component)]
pub struct MeleeAttack {
    pub max_duration: f32,
    pub duration: f32,
    pub range: f32,
    pub attack_power: i32,
}

#[derive(Component)]
pub struct MeleeAttacking {
    progress: f32,
}

pub fn melee_attack(
    mut commands: Commands,
    attacker_query: Query<(&Transform, Entity, &mut MeleeAttack), Without<MeleeAttacking>>,
    mut pl_query: Query<(&Transform, Entity, &mut HP), With<Player>>,
    mut on_hp_changed: MessageWriter<OnHPChanged>,
    time: Res<Time>,
) {
    for (transform, attack_entity, mut melee_attack) in attacker_query {
        let target_query = pl_query
            .iter_mut()
            .filter(|(pl_transform, _, _)| {
                let distance = transform.translation.distance(pl_transform.translation);
                distance < melee_attack.range
            })
            .map(|(_, entity, hp)| (entity, hp));
        if (melee_attack.duration - time.delta_secs()) > 0. {
            if target_query.count() > 0 {
                melee_attack.duration -= time.delta_secs();
            }
        } else {
            for (entity, mut hp) in target_query {
                let last = hp.read_hp();
                hp.set_hp(entity, last - melee_attack.attack_power, &mut on_hp_changed);
            }
            commands
                .entity(attack_entity)
                .insert(MeleeAttacking { progress: 0. });
            melee_attack.duration = melee_attack.max_duration;
        }
    }
}

pub fn attacking_animation(
    mut commands: Commands,
    query: Query<(&mut MeleeAttacking, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut melee_attacking, mut transform, entity) in query {
        melee_attacking.progress += time.delta_secs();
        if melee_attacking.progress > 1. {
            commands.entity(entity).remove::<MeleeAttacking>();
        } else {
            transform.rotation =
                Quat::from_rotation_z((melee_attacking.progress * PI).sin() * (PI / 4.));
        }
    }
}
