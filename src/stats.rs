use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, Res, Time};

#[derive(Component)]
pub struct Stats {
    pub health: u32,
    pub damage: u32,
    pub speed: u32,
    pub attack_speed: f32,
    attack_timer: f32,
    pub xp: u32,
}

impl Stats {
    pub fn new(health: u32, damage: u32, speed: u32, attack_speed: f32, xp: u32) -> Self {
        Stats {
            health,
            damage,
            speed,
            attack_speed,
            attack_timer: 0.,
            xp,
        }
    }

    pub fn can_attack(&self) -> bool {
        self.attack_timer >= self.attack_speed
    }
}

pub fn death_system(mut commands: Commands, query: Query<(Entity, &Stats)>) {
    for (entity, stats) in query.iter() {
        if stats.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn attack_cooldown_system(mut query: Query<&mut Stats>, time: Res<Time>) {
    for mut stats in query.iter_mut() {
        stats.attack_timer += time.delta_seconds();
    }
}
