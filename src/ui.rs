use bevy::prelude::*;

use crate::{
    GameState, GameData
};

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct ScoreText;

// Responds to interaction events with the play button
pub fn play_button(
    mut query: Query<
        (&mut UiImage, &Interaction),
        (Changed<Interaction>, With<PlayButton>),
    >,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
 ) {
    for (mut image, interaction) in &mut query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *image = UiImage(asset_server.load("play-button-hover.png"));
            }
            Interaction::None => {
                *image = UiImage(asset_server.load("play-button.png"));
            }
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
