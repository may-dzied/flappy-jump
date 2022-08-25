use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32
}
impl Position {
    pub fn vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.)
    }
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Weight {
    pub speed: f32
}

#[derive(Component)]
pub struct Jumper {
    pub upward_force: f32
}

pub fn apply_weight(
    mut query: Query<(&mut Velocity, &Weight)>,
    time: Res<Time>
) {
    for (mut velocity, weight) in &mut query {
        velocity.y -= weight.speed * time.delta_seconds();
    }
}

pub fn apply_velocity(
    mut query: Query<(&mut Position, &Velocity)>,
    time: Res<Time>
) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x * time.delta_seconds();
        position.y += velocity.y * time.delta_seconds();
    }
}

pub fn apply_positions(
    mut query: Query<(&mut Transform, &Position)>,
) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}
