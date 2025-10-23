use std::f32::consts::TAU;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::Player;

pub fn movement(
    query: Query<(&mut Transform, &Player)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    for (mut transform, player) in query {
        const PLAYER_SPEED: f32 = 100.;
        if mouse_input.pressed(MouseButton::Left) {
            transform.translation += Vec3::new(player.angle.cos(), player.angle.sin(), 0.)
                * PLAYER_SPEED
                * time.delta_secs();
        }
    }
}

pub fn get_angle_to_mouse(
    player: Query<(&mut Player, &Transform)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let cursor_pos = || -> Option<Vec2> {
        let window = q_window.single().ok()?;
        let (camera, camera_transform) = q_camera.single().ok()?;
        let viewport_position = window.cursor_position()?;
        camera
            .viewport_to_world_2d(camera_transform, viewport_position)
            .ok()
    }();
    for (mut player, transform) in player {
        if let Some(cursor_pos) = cursor_pos {
            let difference = cursor_pos.extend(0.) - transform.translation;
            player.angle = difference.y.atan2(difference.x).rem_euclid(TAU);
        }
    }
}
