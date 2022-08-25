use bevy::prelude::*;

use crate::{
    GameData,
    generator::{Pipe, Coin},
    physics::{Position, Velocity, Weight, Jumper},
    control::Player,
};

pub fn init_camera(
    mut commands: Commands
) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn init_game_data(
    mut commands: Commands,
) {
    commands.insert_resource(GameData {score: 0});
}

pub fn add_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("bird.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(Position {x: 0., y: 0.})
        .insert(Velocity {x: 0., y: 0.})
        .insert(Weight {speed: 800.})
        .insert(Jumper {upward_force: 300.})
        .insert(Player);
}

pub fn cleanup_all(
    mut commands: Commands, 
    query: Query<Entity>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_pipes(
    mut commands: Commands,
    windows: Res<Windows>,
    query: Query<(&Position, Entity), Or<(With<Pipe>, With<Coin>)>>
) {
    let window = windows.get_primary().unwrap();
    for (position, entity) in &query {
        if position.x <= -window.width()/2. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

