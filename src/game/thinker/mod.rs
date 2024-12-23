use std::usize;

use bevy::{ecs::component::ComponentId, prelude::*, utils::{tracing::span::Id, HashMap}};

pub mod thinker;
pub use thinker::*;


pub mod actions;
pub use actions::*;

pub mod scorers;
pub use scorers::*;


pub mod common;
pub use common::*;

pub const SCORER_REST: usize = 10;
pub const SCORER_WANDER: usize = 11;

//#[derive(Debug, PartialEq, Eq, Clone, Reflect)]
// pub enum ScorerList {
//     None, Wander, Rest
// }

// impl Default for ScorerList {
//     fn default() -> Self {
//         Self::None
//     }
// }



pub fn test2(mut cmd: Commands,){

}

pub fn init_entities(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


}

pub struct ThinkerPlugin;

impl Plugin for ThinkerPlugin {
    fn build(&self, app: &mut App) {


        let map = ThinkerBuilder::new(app)
            .register::<WanderAction>( SCORER_WANDER)
            .register::<RestAction>(SCORER_REST)
            //.register_multi_step(SCORER_WANDER)
            .build();

        app
        .register_type::<Score>()
        .register_type::<Action>()
        .register_type::<AbilityStats>()
        .register_type::<MovementAction>()

        //.insert_resource(map)
        //.init_resource(map)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, score_management_system)

        .add_systems(FixedUpdate, 
            (
                    rest_scorer_system::<SCORER_REST>,
                    wander_scorer_system::<SCORER_WANDER>,


                    wander_action_system,
                    rest_action_system,

                    //common
                    move_system,
        ));
        // app

        //     .add_systems(Update, move |cmd: Commands| {
        //         // call our function from inside the closure
        //         my_system(cmd, &config);
        //     })



        //     .add_systems(Update, (
        //         camera_move,
        //         test_system,
        //         //p.test_system
        //     ));

        // let mut world = app.world();
        // let id = world.register_component::<Action>();
        // let id2 = id.
        // let mut services: HashMap<String, Box<dyn MyTrait>> = HashMap::new();
        // services.insert("abhi".to_string(), Box::new(Bar));
        // services.insert("rust".to_string(), Box::new(Foo));
        //  = HashMap::from([
        //     ("abhi".to_string(), Box::new(Bar)),
        //     ("rust".to_string(), Box::new(Foo))
        // ]);

        //let mut s: HashMap<String, Box<dyn Component<Storage=SparseStorage>>> = HashMap::new();
        // s.insert("rust".to_string(), Box::new(Test2));
        // s.insert("rust2".to_string(), Box::new(Test));
    }
}

fn setup(mut cmd: Commands,){
    //cmd.init_resource::<ActionScorerMap>();
}


