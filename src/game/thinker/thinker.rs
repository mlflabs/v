use std::{ usize};

use bevy::{ecs::component::{self, ComponentId}, prelude::*, reflect::Map, utils::HashMap};
use bevy::prelude::Component;



#[derive(Component, Reflect, Default, Debug)]
pub struct ThinkerTag;


#[derive(Component, Reflect, Default, Debug)]
pub struct ThinkerExecutingActionTag;




#[derive(Component, Default, Debug, Reflect)]
pub enum ActionState {
    #[default] Init, Running, Cleanup, Finished
}

#[derive(Component, Default, Debug, Reflect)]
pub struct Action {
    pub state: ActionState,
    pub action: usize,
}






#[derive(Debug, PartialEq, Eq, Reflect)]
pub enum ScorerStep {
    Init, Evaluating, AssigningAction, CleaningUp, Finished
}

impl Default for ScorerStep {
    fn default() -> Self {
        Self::Evaluating
    }
}


#[derive(Component, Debug, Default, Reflect)]
pub struct Score {
    pub value: f32,
    pub step: ScorerStep,
    pub scorer: usize,
    pub previous_winner: usize,
    pub previous_winner2: usize
}




#[derive(Resource, Clone, Reflect)]
pub struct ActionScorerMap {
    // scorer Vec<Actiosn>
    pub map: HashMap<usize, Vec<usize>>,
    //pub v: Vec<Box<dyn Component<Storage=SparseStorage>>>
}

impl Default for ActionScorerMap {
    fn default() -> Self {
        ActionScorerMap { map: HashMap::new() }
    }
}


pub struct ThinkerBuilder<'a> {
    app: &'a mut App,
    pub map: ActionScorerMap
}


impl<'a> ThinkerBuilder<'a> {
    

    pub fn new(app: &'a mut App) -> Self {
        Self {app, map: ActionScorerMap::default() }
    }

    pub fn register_multi_step<T:Component+Sized>(
        &mut self, id:usize ) -> &mut Self {
        //let id = comps.
        // let id1 = self.app.world_mut().register_component::<T>();
        // let comp = self.app.world_mut().register_component::<T>();
        // //self.map.map.insert(comp,ids);
        return self;
    }

    pub fn register<T:Component>(&mut self, id:usize, )->&mut Self {
        
        let comp = self.app.world_mut().register_component::<T>();

        if self.map.map.contains_key(&id){
            let v = self.map.map.get_mut(&id).unwrap();
            v.push(comp.index());
            //self.map.map.insert(id, v.   );

        }
        else {
            self.map.map.insert(id, vec![comp.index()]);
        }
        


        return self;
    }
    pub fn build(&mut self)-> ActionScorerMap {
        println!("Building resource");
        self.app.world_mut().insert_resource(self.map.clone());
        return self.map.clone();
        // for (id, comp) in &self.map.map {
        //     self.app.world_mut().insert_resource(self.map.clone());
        // }
    }

}





pub fn score_management_system(
    mut cmd: Commands,
    map: Res<ActionScorerMap>,
    mut scorers: Query<(Entity, &mut Score, &mut Action, Option<&ThinkerExecutingActionTag>)>,
){

    for (e, mut score, mut action,thinker) in scorers.iter_mut() {
        //Are we executing or picking
        // if let i = &map {
        //    println!("res:::::::::::::::::{:?}", i.map);
        // }

        if let Some(_thinker) = thinker {
            match action.state {
                ActionState::Init => {
                    println!("thinker Action, Initial");
                    //ActionState::Running
                    action.state = ActionState::Running;
                },
                ActionState::Running => {
                    //ActionState::Cleanup
                },
                ActionState::Cleanup => {
                    println!("AcitonState::Cleanup");
                    let i = map.map.get(&score.scorer).unwrap();

                    //let mut index = 0;
                    let mut idx = 0;

                    for id in i.iter(){
                        println!("1, {:?}, {:?}", id, &action.action);
                        println!("Score: {:?}", &score);
                        println!("Action: {:?}", &action);
                        if id == &action.action {
                            println!("2");
                            if idx == i.len() - 1 {
                                println!("3");
                                //last record
                                cmd.entity(e).remove::<ThinkerExecutingActionTag>();
                                let c = ComponentId::new(*id);
                                cmd.entity(e).remove_by_id(c);
                            }
                            else {
                                println!("4");
                                //more action to go
                                let c = ComponentId::new(action.action);
                                cmd.entity(e).remove_by_id(c);

                                let cc = ComponentId::new(i[idx + 1]);
                                //cmd.entity(e).insert_by_id(cc, {});
                                //unsafe { cmd.entity(e).insert_by_id(cc, {}) };

                                //let ttt = RestAction::default();
                                //let sdf = cmd.(ttt);
                                
                            }
                        }
                        else {
                            println!("5: {:?}", idx);
                            idx += 1;
                        }
                    }

                    //let t = map.map.get(&score.scorer);

                    
                    
    
    
                    //ActionState::Init
                },
                _ => {

                }
                //ActionState::Finished => ActionState::Init
            };
        }
        else {
            score.step = match score.step {
                ScorerStep::Init => {
                    //println!("Thinker INIT");
                    score.previous_winner2 = score.previous_winner.clone();
                    score.previous_winner = score.scorer.clone();
                    score.value = 0.;
                    score.scorer = usize::default();
    
                    ScorerStep::Evaluating
                },
                ScorerStep::Evaluating => ScorerStep::AssigningAction,
                ScorerStep::AssigningAction => {
                    println!("AssigningAction");
                    println!("Score: {:?}", score);
                    let i = map.map.get(&score.scorer);
                    if let Some(v)  = i {
                        let cc = ComponentId::new(v[0]);
                        unsafe { cmd.entity(e).insert_by_id(cc, {}) };
                        action.action = v[0];

                        ScorerStep::CleaningUp
                    }
                    else {
                        println!("Scorer doesn't point to any action");
                        //nothing found, lets go back and try again
                        ScorerStep::Init
                    }
                }
                ScorerStep::CleaningUp => {
                    println!("Thinker cleanup");
                    cmd.entity(e).insert(ThinkerExecutingActionTag);
                    ScorerStep::Finished
                },
                ScorerStep::Finished => ScorerStep::Init
            
            }
        }
        
    }
}












