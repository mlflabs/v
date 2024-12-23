use bevy::prelude::*;

use crate::MAX_DISTANCE;



#[derive(Debug, PartialEq, Eq, Clone, Default, Reflect)]
pub enum MovementState {
    #[default]
    None,
    Success,
    Fail,
    Executing,
}

// impl Default for MovementState {
//     fn default() -> Self { MovementState::None }
// }

#[derive(Component, Reflect, Default, Debug)]
#[require(AbilityStats)]
pub struct MovementAction {
    pub dest: Vec3,
    pub state: MovementState,
}

#[derive(Component, Reflect, Debug)]
pub struct AbilityStats {
    pub speed:f32,
}

impl Default for AbilityStats {
    fn default() -> Self { 
        AbilityStats {
            speed: 1.0,
        }
    }
}




pub fn move_system(
    //mut cmd: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut MovementAction, &mut Transform, &AbilityStats)>
){
    for (e, mut movement, mut transform, stats) in query.iter_mut() {
        if movement.state == MovementState::Executing {

             let delta = movement.dest - transform.translation;
             let distance = delta.length();

             if distance > MAX_DISTANCE {
                 
                 // How far can we travel during this frame?
                 let step_size = time.delta_secs() * stats.speed;
                 // Travel towards the water-source position, but make sure to not overstep it.
                 let step = delta.normalize() * step_size.min(distance);

                 // Move the actor.
                 transform.translation = transform.translation + step;
             } else {
                 // We're within the required distance! We can declare success.

                 info!("We got there!");
                 movement.state = MovementState::Success;

             } 
        }
        else if movement.state == MovementState::None {
            return;
        }
        else if movement.state == MovementState::Fail {
            return;
        }
    }
}





