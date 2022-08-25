use bevy::{
    prelude::*,
    sprite::collide_aabb::collide
};

use crate::{
    GameState, GameData,
    PIPE_SCALE, PIPE_WIDTH, PIPE_HEIGHT,
    control::Player,
    physics::Position,
    generator::{Pipe, Coin}
};

// Detects when player collides with a pipe or a coin
pub fn check_collisions(
    players: Query<&Position, With<Player>>,
    pipes: Query<&Position, With<Pipe>>,
    coins: Query<(&Position, Entity), With<Coin>>,
    mut state: ResMut<State<GameState>>,
    mut data: ResMut<GameData>,
    mut commands: Commands
) {
    let player = players.single();
    for pipe in &pipes {
        let collision = collide(
            player.vec3(), 
            Vec2::new(32., 32.),
            pipe.vec3(),
            Vec2::new(PIPE_WIDTH*PIPE_SCALE, PIPE_HEIGHT*PIPE_SCALE)
        );
        if collision.is_some() {
            state.set(GameState::GameOver).unwrap();
        }
    }
    for (coin, e) in &coins {
        let collision = collide(
            player.vec3(), 
            Vec2::new(32., 32.),
            coin.vec3(),
            Vec2::new(32., 32.)
        );
        if collision.is_some() {
            data.score += 1;
            commands.entity(e).despawn_recursive();
        }
    }
}

// Kills the player if they go off the screen
pub fn check_bounds(
    players: Query<&Position, With<Player>>,
    windows: Res<Windows>,
    mut state: ResMut<State<GameState>>,
) {
    let player = players.single();
    let window = windows.get_primary().unwrap();
    if player.y < -window.height()/2. || player.y > window.height()/2. {
        state.set(GameState::GameOver).unwrap();
    }
}
