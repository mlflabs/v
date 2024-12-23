use bevy::prelude::*;

use crate::{Action, ActionState, ThinkerExecutingActionTag};



#[derive(Component, Reflect, Default, Debug)]
pub struct RestAction;


pub fn rest_action_system(
    mut cmd: Commands,
    mut actions: Query<(Entity, &mut Action), (With<RestAction>, With<ThinkerExecutingActionTag>)>
){
    for (e, mut action) in actions.iter_mut() {

        action.state = match action.state {
            ActionState::Init => {
                println!("ACTION...........Rest, Initial");
                ActionState::Running
            },
            ActionState::Running => {
                ActionState::Running
            },
            ActionState::Cleanup => {
                //cmd.entity(e).remove::<RestAction>();
                cmd.entity(e).remove::<ThinkerExecutingActionTag>();
                ActionState::Init
            },
            _ => {
                println!("Found not support option");
                ActionState::Finished
            }
        };
    }
}