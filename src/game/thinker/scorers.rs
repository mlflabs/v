use std::string;

use bevy::{ecs::component::ComponentId, prelude::*};

use super::{RestAction, Score, ScorerStep, ThinkerExecutingActionTag};



#[derive(Component, Reflect, Default, Debug)]
pub struct WanderScorerTag;


pub fn wander_scorer_system<const id: usize>(
    mut cmd: Commands,
    mut scorers: Query<(Entity, &mut Score), (With<WanderScorerTag>, Without<ThinkerExecutingActionTag>)>
){
    for (e, mut score) in scorers.iter_mut() {
        match score.step {
            ScorerStep::Evaluating=> {
                println!("--------------- wander eval");
                if score.previous_winner == id 
                {
                    if score.value < 0.5 {
                        score.value = 0.5;
                        score.scorer = id;
                    }
                    
                }
                else {
                    if score.value < 1. {
                        score.value = 1.;
                        score.scorer = id;
                    }
                }
            },
            _ => {},
            //_ => println!("Not supported step")
        };
    }
}





#[derive(Component, Reflect, Default, Debug)]
pub struct RestScorerTag;






pub fn rest_scorer_system<const id: usize>(
    mut cmd: Commands,
    mut scorers: Query<(Entity, &mut Score), (With<RestScorerTag>, Without<ThinkerExecutingActionTag>)>
){
    for (e, mut score) in scorers.iter_mut() {
        if score.step == ScorerStep::Evaluating {
                println!("-------------------- rest eval");
                if  score.value < 0.8 {
                    score.value = 0.8;
                    score.scorer = id;
                }
        }
        else if score.step == ScorerStep::CleaningUp {
            // if score.scorer == ScorerList::Rest {
            //     cmd.entity(e).insert(RestAction);
            // }
        }
    }
}


