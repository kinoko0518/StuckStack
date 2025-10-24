use bevy::prelude::*;
use rand::Rng;

pub struct AttractablePlugin;

impl Plugin for AttractablePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<OnHPChanged>()
            .add_message::<OnDied>()
            .add_systems(Update, (hp_changed_popup, hp_popup));
    }
}

#[derive(Component)]
pub struct HP {
    health: i32,
}

#[derive(Message)]
pub struct OnDied {
    entity: Entity,
}

#[derive(Message)]
pub struct OnHPChanged {
    entity: Entity,
    from: i32,
    to: i32,
}

impl HP {
    pub fn set_hp(
        &mut self,
        entity: Entity,
        value: i32,
        on_hp_changed: &mut MessageWriter<OnHPChanged>,
    ) {
        on_hp_changed.write(OnHPChanged {
            entity: entity,
            from: self.health,
            to: value,
        });
        self.health = value;
    }
    pub fn read_hp(&self) -> i32 {
        self.health
    }
    pub fn new(hp: i32) -> Self {
        Self { health: hp }
    }
}

#[derive(Component)]
pub struct HPPopup {
    progress: f32,
}

pub fn hp_changed_popup(
    mut commands: Commands,
    mut hp_changed: MessageReader<OnHPChanged>,
    attractable: Query<&Transform, With<HP>>,
) {
    for event in hp_changed.read() {
        let difference = event.to - event.from;
        let mut rng = rand::rng();
        const DISCRATE: f32 = 50.;
        if let Ok(transform) = attractable.get(event.entity) {
            commands.spawn((
                HPPopup { progress: 0. },
                Transform::from_translation(
                    transform.translation
                        + Vec3::new(
                            rng.random_range((-DISCRATE)..DISCRATE),
                            rng.random_range((-DISCRATE)..DISCRATE),
                            0.,
                        ),
                ),
                Text2d::from(format!("{}", difference.abs())),
                TextColor::from(Color::srgb_u8(247, 2, 92)),
            ));
        }
    }
}

pub fn hp_popup(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(&mut HPPopup, &mut Transform, Entity)>,
) {
    for (mut popup, mut transform, entity) in query {
        transform.translation.y += popup.progress;
        popup.progress += time.delta_secs();
        if popup.progress > 1. {
            commands.entity(entity).despawn();
        }
    }
}
