use bevy::{
    prelude::*,
    window::WindowResized
};

use crate::{
    GameState, GameData
};

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Background;

// Responds to interaction events with the play button
pub fn play_button(
    query: Query<
        &Interaction,
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut state: ResMut<State<GameState>>,
 ) {
    for interaction in &query {
        if let Interaction::Clicked = *interaction {
            state.set(GameState::InGame).unwrap();
        }
    }
}

// Keeps the score text updated
pub fn update_score_text(
    mut query: Query<&mut Text, With<ScoreText>>,
    data: Res<GameData>,
) {
    for mut text in &mut query {
        text.sections[0].value = data.score.to_string();
    }
}

pub fn show_background(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>
) {
    let window = windows.get_primary().unwrap();
    let width_scale = window.width() / 32.;
    let height_scale = window.height() / 32.;
    let mut background_transform = Transform::from_scale(Vec3::new(width_scale, height_scale, 1.));
    background_transform.translation.z = 50.;
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: background_transform,
            ..default()
        })
        .insert(Background);
}

pub fn resize_background(
    resize_event: Res<Events<WindowResized>>,
    mut query: Query<&mut Transform, With<Background>>
) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        for mut transform in &mut query {
            transform.scale.x = e.width / 32.;
            transform.scale.y = e.height / 32.;
        }
    }
}

// Adds the play button to the screen
pub fn add_menu_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            image: UiImage(asset_server.load("play-button.png")), 
            ..default()
        })
        .insert(PlayButton);
}

// Shows a game over message
pub fn show_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("font.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::BLACK,
    };
    let text_alignment = TextAlignment::CENTER;

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("GAME OVER", text_style).with_alignment(text_alignment),
            transform: Transform::from_xyz(0., 100., 0.),
            ..default()
        })
        .insert(ScoreText);
}

// Shows the game logo
pub fn show_menu_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("font.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::BLACK,
    };
    let text_alignment = TextAlignment::CENTER;

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("Flappy Jump", text_style).with_alignment(text_alignment),
            transform: Transform::from_xyz(0., 100., 0.),
            ..default()
        })
        .insert(ScoreText);
}

// Creates a text bundle which displays the score at the time of creation
pub fn show_score(
    mut commands: Commands,
    data: Res<GameData>,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("font.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::BLACK,
    };
    let text_alignment = TextAlignment::CENTER;

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(data.score.to_string(), text_style).with_alignment(text_alignment),
            ..default()
        })
        .insert(ScoreText);
}
