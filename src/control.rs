use bevy::prelude::*;

use crate::physics::{
    Velocity,
    Jumper
};

const JUMP_KEYS: [KeyCode; 2] = [KeyCode::Space, KeyCode::Return];

#[derive(Component)]
pub struct Player;

pub fn jump(
    mut query: Query<(&mut Velocity, &Jumper)>,
    input: Res<Input<KeyCode>>
) {
    for (mut velocity, jumper) in &mut query {
        if JUMP_KEYS.iter().any(|key| input.just_pressed(*key)) {
            if velocity.y <= 0. {
                velocity.y = jumper.upward_force;
            } else {
                let added = velocity.y + jumper.upward_force;
                let double = jumper.upward_force * 2.;
                if added < double {
                    velocity.y = added;
                } else {
                    velocity.y = double;
                }
            }
        }
    }
}
