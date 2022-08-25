#![allow(clippy::type_complexity)]

pub mod physics;
pub mod manager;
pub mod ui;
pub mod generator;
pub mod control;
pub mod collision;

use bevy::prelude::*;

pub const GAP: f32 = 200.;
pub const PADDING: f32 = 100.;
pub const PIPE_WIDTH: f32 = 32.;
pub const PIPE_HEIGHT: f32 = 400.;
pub const PIPE_SCALE: f32 = 3.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    GameOver,
}
pub struct GameData {
    pub score: u64
}

pub fn main() {
    App::new()

        .add_state(GameState::MainMenu)
        .add_plugins(DefaultPlugins)

        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(manager::init_camera)
                .with_system(ui::show_menu_screen)
                .with_system(ui::add_menu_buttons)
                .with_system(manager::init_game_data)
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(ui::play_button)
        )

        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(manager::cleanup_all)
                .with_system(manager::init_camera)
                .with_system(manager::add_player)
                .with_system(ui::show_score)
                .with_system(generator::start)
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(physics::apply_weight)
                .with_system(physics::apply_velocity)
                .with_system(physics::apply_positions)
                .with_system(control::jump)
                .with_system(collision::check_collisions)
                .with_system(collision::check_bounds)
                .with_system(generator::spawn_pipes)
                .with_system(manager::cleanup_pipes)
                .with_system(ui::update_score_text)
        )

        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(manager::cleanup_all)
                .with_system(manager::init_camera)
                .with_system(ui::show_score)
                .with_system(ui::show_game_over)
        )

        .run()
}
