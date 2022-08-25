use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    PADDING, GAP, PIPE_SCALE, PIPE_HEIGHT,
    physics::{Position, Velocity}
};

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct Coin;

pub struct PipeSpawnSchedule {
    pub time_left: f32
}

pub fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut schedule: ResMut<PipeSpawnSchedule>,
) {
    schedule.time_left -= time.delta_seconds();
    if schedule.time_left <= 0. {
        let window = windows.get_primary().unwrap();
        let mut rng = rand::thread_rng();

        let gap_min = -window.height()/2.0 + GAP/2. + PADDING;
        let gap_max = window.height()/2.0 - GAP/2. - PADDING;
        let gap = rng.gen_range(gap_min..gap_max);

        let bottom_pipe_top = gap - GAP/2.;
        let bottom_pipe_y = bottom_pipe_top - PIPE_HEIGHT*PIPE_SCALE/2.;

        let top_pipe_bottom = gap + GAP/2.;
        let top_pipe_y = top_pipe_bottom + PIPE_HEIGHT*PIPE_SCALE/2.;

        // Spawn bottom pipe
        let position = Position {
            x: window.width() / 2., 
            y: bottom_pipe_y
        };
        let mut pipe_transform = Transform::from_xyz(position.x, position.y, 0.);
        pipe_transform.scale = Vec3::new(3., 3., 1.);
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("pipe.png"),
                transform: pipe_transform,
                ..default()
            })
            .insert(position)
            .insert(Velocity {x: -160., y: 0.})
            .insert(Pipe);

        // Spawn top pipe
        let position = Position {
            x: window.width() / 2., 
            y: top_pipe_y
        };
        let mut pipe_transform = Transform::from_xyz(position.x, position.y, 0.);
        pipe_transform.scale = Vec3::new(3., 3., 1.);
        pipe_transform.rotation = Quat::from_rotation_z(180.0f32.to_radians());
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("pipe.png"),
                transform: pipe_transform,
                ..default()
            })
            .insert(position)
            .insert(Velocity {x: -160., y: 0.})
            .insert(Pipe);

        // Spawn coin
        let position = Position {
            x: window.width() / 2., 
            y: gap
        };
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("coin.png"),
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..default()
            })
            .insert(position)
            .insert(Velocity {x: -160., y: 0.})
            .insert(Coin);

        schedule.time_left = 3.;
    }
}

pub fn start(
    mut commands: Commands,
) {
    commands.insert_resource(PipeSpawnSchedule {time_left: 2.});
}
