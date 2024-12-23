use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::utils::tracing::{debug, trace};
use bevy_voxel_world::prelude::*;
use big_brain::prelude::*;
use std::sync::Arc;


use brain::{MyBrain};


mod brain;


mod game;
use game::*;


// Declare materials as consts for convenience
// This can also be an enum or other type, see the `textures_custom_idx.rs` example
const SNOWY_BRICK: u8 = 0;
const FULL_BRICK: u8 = 1;
const GRASS: u8 = 2;
const GROUND: u8 = 3;

#[derive(Resource, Clone, Default)]
struct MyMainWorld;

impl VoxelWorldConfig for MyMainWorld {
    type MaterialIndex = u8;

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|vox_mat| match vox_mat {
            GROUND => [0, 1, 2],
            SNOWY_BRICK => [3, 3, 3],
            FULL_BRICK => [2, 2, 2],
            GRASS | _ => [5, 6, 6],
        })
    }

    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(("textures.png".into(), 10))
    }
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (400., 400.).into(),
                    title: "VectorRun".to_string(),
                    resizable: true,
                    position: WindowPosition::At(IVec2::new(0, 1050)),
                    //position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..Default::default()
                    }),
                ..Default::default()
            })
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            })
        )
        .add_plugins(WorldInspectorPlugin::new())

        .register_type::<Stanima>()

        .add_plugins(MyBrain)
        // We can specify a custom texture when initializing the plugin.
        // This should just be a path to an image in your assets folder.
        .add_plugins(VoxelWorldPlugin::with_config(MyMainWorld))
        .add_systems(Startup, (setup, create_voxel_scene).chain())


        //.add_plugins(BigBrainPlugin::new(PreUpdate))
        //.add_systems(Startup, init_entities)
        //.add_systems(Update, thirst_system)
        //.add_systems(PreUpdate, drink_action_system.in_set(BigBrainSet::Actions))
        // .add_systems(
        //     PreUpdate,
        //     (
        //         drink_action_system.in_set(BigBrainSet::Actions),
        //         thirsty_scorer_system.in_set(BigBrainSet::Scorers),
        //     ),
        // )
        .run();
}

#[derive(Component, Debug, Reflect)]
pub struct Stanima {
    pub value: f32,
    pub per_second: f32,
}

impl Stanima {
    pub fn new(value: f32, decrement_per_second: f32) -> Self {
        Self { value, per_second : decrement_per_second }
    }
}

#[derive(Clone, Component, Debug, Default, ActionBuilder)]
pub struct Rest {
    pub per_second: f32,
    pub until: f32,
}

#[derive(Component, Debug)]
pub struct Wander {
    //pub destination: Vec3<f32>,
}

pub fn stanima_system(time: Res<Time>, mut query: Query<&mut Stanima>) {
    for mut stanima in &mut query {
        stanima.value -= stanima.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
        if stanima.value <= 0.0 {
            stanima.value = 0.0;
        }
        trace!("Stanima: {}", stanima.value);
    }
}

pub fn wander_system(time: Res<Time>, mut wanders: Query<&mut Wander>) {
    for mut wander in &mut wanders {
        
        

        trace!("Wandering");
    }
}


#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Tierd;


pub fn tierd_scorer_system(
    stanimas: Query<&Stanima>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Tierd>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(stanima) = stanimas.get(*actor) {
            score.set((stanima.value-100.).abs() / 100.0);
        }
    }
}



fn wander_action_system(
    time: Res<Time>,
    mut wanders: Query<&mut Wander>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Wander, &ActionSpan)>,
) {
    for (Actor(actor), mut state, wander, span) in &mut query {
        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok(mut wander) = wanders.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    info!("Time to drink some water!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Wandering...");
                    
  
                    //only after its at destination
                    *state = ActionState::Success;
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    info!("Action was cancelled. Considering this a failure.");
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}




fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        // This tells bevy_voxel_world to use this cameras transform to calculate spawning area
        VoxelWorldCamera::<MyMainWorld>::default(),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));



    commands.spawn((
        Name::new("Player"),
        Stanima::new(75.0, 2.0),
        Thinker::build()
         .label("My Thinker2")
         .picker(Highest)
         .when(Tierd, Rest {
            per_second: 5.0,
            until: 20.
         }),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 0))),
        Transform::from_xyz(2.5, 0.5, 0.5),
    ));
}

fn create_voxel_scene(mut voxel_world: VoxelWorld<MyMainWorld>) {
    // Then we can use the `u8` consts to specify the type of voxel

    // 20 by 20 floor
    for x in -20..20 {
        for z in -20..20 {
            voxel_world.set_voxel(IVec3::new(x, -1, z), WorldVoxel::Solid(GRASS));
            // Grassy floor
        }
    }

    // Some bricks
    voxel_world.set_voxel(IVec3::new(0, 0, 0), WorldVoxel::Solid(SNOWY_BRICK));
    voxel_world.set_voxel(IVec3::new(0, 1, 0), WorldVoxel::Solid(SNOWY_BRICK));
    voxel_world.set_voxel(IVec3::new(1, 0, 0), WorldVoxel::Solid(SNOWY_BRICK));
    voxel_world.set_voxel(IVec3::new(0, 0, 1), WorldVoxel::Solid(SNOWY_BRICK));
    voxel_world.set_voxel(IVec3::new(-5, 0, -5), WorldVoxel::Solid(SNOWY_BRICK));
    voxel_world.set_voxel(IVec3::new(5, 0, 5), WorldVoxel::Solid(SNOWY_BRICK));
}

#[derive(Component, Debug)]
pub struct Thirst {
    pub per_second: f32,
    pub thirst: f32,
}

impl Thirst {
    pub fn new(thirst: f32, per_second: f32) -> Self {
        Self { thirst, per_second }
    }
}

pub fn thirst_system(time: Res<Time>, mut thirsts: Query<&mut Thirst>) {
    for mut thirst in &mut thirsts {
        thirst.thirst += thirst.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
        if thirst.thirst >= 100.0 {
            thirst.thirst = 100.0;
        }
        trace!("Thirst: {}", thirst.thirst);
    }
}





// Now that we have all that defined, it's time to add a Thinker to an entity!
// The Thinker is the actual "brain" behind all the AI. Every entity you want
// to have AI behavior should have one *or more* Thinkers attached to it.
pub fn init_entities(mut cmd: Commands) {
    // Create the entity and throw the Thirst component in there. Nothing special here.
    cmd.spawn((
        Thirst::new(75.0, 2.0),
        Thinker::build()
            .label("My Thinker")
            .picker(FirstToScore { threshold: 0.8 })
            // Technically these are supposed to be ActionBuilders and
            // ScorerBuilders, but our Clone impls simplify our code here.
            .when(Thirsty, Drink {
                until: 70.0,
                per_second: 5.0,
            }),
    ));
}




// The second step is to define an action. What can the AI do, and how does it
// do it? This is the first bit involving Big Brain itself, and there's a few
// pieces you need:
//
// 1. An Action Component. This is just a plain Component we will query
//    against later.
// 2. An ActionBuilder. This is anything that implements the ActionBuilder
//    trait.
// 3. A System that will run Action code.
//
// These actions will be spawned and queued by the game engine when their
// conditions trigger (we'll configure what these are later).
//
// In most cases, the ActionBuilder just attaches the Action component to the
// actor entity. In this case, you can use the derive macro `ActionBuilder`
// to make your Action Component implement the ActionBuilder trait.
// You need your type to implement Clone and Debug (necessary for ActionBuilder)
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Drink {
    until: f32,
    per_second: f32,
}

// Action systems execute according to a state machine, where the states are
// labeled by ActionState.
fn drink_action_system(
    time: Res<Time>,
    mut thirsts: Query<&mut Thirst>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>,
) {
    for (Actor(actor), mut state, drink, span) in &mut query {
        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok(mut thirst) = thirsts.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    info!("Time to drink some water!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Drinking...");
                    thirst.thirst -=
                        drink.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
                    if thirst.thirst <= drink.until {
                        // To "finish" an action, we set its state to Success or
                        // Failure.
                        info!("Done drinking water");
                        *state = ActionState::Success;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    info!("Action was cancelled. Considering this a failure.");
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}



#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Thirsty;


// Looks familiar? It's a lot like Actions!
pub fn thirsty_scorer_system(
    thirsts: Query<&Thirst>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Thirsty>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(thirst) = thirsts.get(*actor) {
            // This is really what the job of a Scorer is. To calculate a
            // generic "Utility" score that the Big Brain engine will compare
            // against others, over time, and use to make decisions. This is
            // generally "the higher the better", and "first across the finish
            // line", but that's all configurable using Pickers!
            //
            // The score here must be between 0.0 and 1.0.
            score.set(thirst.thirst / 100.0);
            if thirst.thirst >= 80.0 {
                span.span().in_scope(|| {
                    info!("Thirst above threshold! Score: {}", thirst.thirst / 100.0)
                });
            }
        }
    }
}
