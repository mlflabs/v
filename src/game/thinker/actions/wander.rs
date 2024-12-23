use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand::Rng;
use crate::{Action, ActionState, MovementAction, MovementState, ThinkerExecutingActionTag};



#[derive(Component, Reflect, Default, Debug)]
pub struct WanderAction;


pub fn wander_action_system(
    mut cmd: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut actions: Query<(Entity, &mut Action, &mut MovementAction, &Transform), (With<WanderAction>, With<ThinkerExecutingActionTag>)>
){
    for (e, mut action, mut move_action, trans) in actions.iter_mut() {
        match action.state {
            ActionState::Init => {
                println!("Wander, Initial");

                let dist = Vec3::new(
                    rng.gen_range(-10.0..10.0),
                    0.,
                    rng.gen_range(-10.0..10.0)
                );
                move_action.dest = trans.translation + dist;
                move_action.state = MovementState::Executing;
                action.state = ActionState::Running;
            },
            ActionState::Running => {

                if move_action.state == MovementState::Executing {
                    return;
                }

                action.state = ActionState::Cleanup;
            },
            _ => {

            }
        };
    }
}


