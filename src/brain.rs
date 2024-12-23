use bevy::math::{vec2, vec3};
use bevy::{color::palettes::css, prelude::*};
use bevy::utils::tracing::{debug, trace};
use big_brain::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use bevy_rand::prelude::EntropyPlugin;
use rand::prelude::*;



pub struct MyBrain;
 
impl Plugin for MyBrain {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Thirst>()
            .register_type::<Actor>()
            .register_type::<Thinker>()
            .register_type::<Action>()
            .register_type::<Score>()
            .register_type::<ActionState>()


            .add_plugins(EntropyPlugin::<WyRand>::default())
            .add_plugins(BigBrainPlugin::new(PreUpdate))
            .add_systems(Startup, init_entities)
            .add_systems(Update, thirst_system)
            .add_systems(
                PreUpdate,
                (drink_action_system, 
                         wander_system,
                         move_to_water_source_action_system).in_set(BigBrainSet::Actions),
            )
            .add_systems(First, (thirsty_scorer_system, wander_scorer_system));
    }
}




pub fn init_entities(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    cmd.spawn((
        Name::new("Water"),
        WaterSource,
        Position {
            position: Vec2::new(7.0, 7.0),
        },
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::Srgba(css::BLUE))),
        Transform::from_xyz(7., 0.5, 7.),
    ));

    cmd.spawn((
        Name::new("Water"),
        WaterSource,
        Position {
            position: Vec2::new(-3.0, -3.0),
        },
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::Srgba(css::BLUE))),
        Transform::from_xyz(-3., 0.5, -3.),
    ));


    let move_and_drink = Steps::build()
        .label("MoveAndDrink")
        // ...move to the water source...
        .step(MoveToWaterSource { speed: 1.0 })
        // ...and then drink.
        .step(Drink { per_second: 10.0 });

    // Build the thinker
    let thinker = Thinker::build()
        .label("ThirstyThinker")
        // We don't do anything unless we're thirsty enough.
        .picker(Highest)
        .when(WanderScorer, WanderAction::default())
        .when(Thirsty, move_and_drink);


    cmd.spawn((
            Name::new("Player"),
            Thirst::new(75.0, 2.0),
            Position {
                position: Vec2::new(0.0, 0.0),
            },
            thinker.clone(),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::Srgba(css::RED))),
            Transform::from_xyz(0., 0.5, 0.),
    ));


    cmd.spawn((
        Name::new("Player"),
        Thirst::new(75.0, 2.0),
        Position {
            position: Vec2::new(2.0, 2.0),
        },
        thinker.clone(),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::Srgba(css::RED))),
        Transform::from_xyz(2., 0.5, 2.),
    ));
}
 










/// First, we make a simple Position component.
#[derive(Component, Debug, Copy, Clone)]
pub struct Position {
    pub position: Vec2,
}

/// A marker component for an entity that describes a water source.
#[derive(Component, Debug)]
pub struct WaterSource;

/// We steal the Thirst component from the thirst example.
#[derive(Component, Debug, Reflect)]
pub struct Thirst {
    /// How much thirstier the entity gets over time.
    pub per_second: f32,
    /// How much thirst the entity currently has.
    pub thirst: f32,
}

impl Thirst {
    pub fn new(thirst: f32, per_second: f32) -> Self {
        Self { thirst, per_second }
    }
}

/// A simple system that just pushes the thirst value up over time.
/// Just a plain old Bevy system, big-brain is not involved yet.
pub fn thirst_system(time: Res<Time>, mut thirsts: Query<&mut Thirst>) {
    for mut thirst in &mut thirsts {
        thirst.thirst += thirst.per_second * time.delta_secs();

        // Thirst is capped at 100.0
        if thirst.thirst >= 100.0 {
            thirst.thirst = 100.0;
        }

        trace!("Thirst: {}", thirst.thirst);
    }
}


#[derive(Clone, Default, Component, Debug, ActionBuilder)]
pub struct WanderAction {
    pub dest: Vec2,
    pub speed: f32,
}



fn wander_system(
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    // Find all water sources
    // We use Without to make disjoint queries.
    mut positions: Query<(&mut Position, &mut Transform), Without<WaterSource>>,
    // A query on all current MoveToWaterSource actions.
    mut action_query: Query<(&Actor, &mut ActionState, &mut WanderAction, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (actor, 
            mut action_state, 
            mut wander, 
            span) in &mut action_query {
        let _guard = span.span().enter();

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                info!("Let's wander around!");
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                let (mut actor_position, mut transform) 
                    = positions.get_mut(actor.0).expect("actor has no position");

                
                if wander.speed == 0.0 {
                    wander.dest = actor_position.position + 
                        Vec2::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));  
                    wander.speed = rng.gen_range(1.0..3.5);
                }

                // Find how far we are from it.
                let delta = wander.dest - actor_position.position;

                let distance = delta.length();

                trace!("Distance: {}", distance);

                if distance > MAX_DISTANCE {
                    // We're still too far, take a step toward it!

                    trace!("Stepping closer.");

                    // How far can we travel during this frame?
                    let step_size = time.delta_secs() * wander.speed;
                    // Travel towards the water-source position, but make sure to not overstep it.
                    let step = delta.normalize() * step_size.min(distance);

                    // Move the actor.
                    actor_position.position += step;
                    transform.translation = Vec3::new(  actor_position.position.x, 
                                                        transform.translation.y, 
                                                        actor_position.position.y);
                } else {
                    // We're within the required distance! We can declare success.

                    info!("We got there!");

                    // The action will be cleaned up automatically.
                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                // Always treat cancellations, or we might keep doing this forever!
                // You don't need to terminate immediately, by the way, this is only a flag that
                // the cancellation has been requested. If the actor is balancing on a tightrope,
                // for instance, you may let them walk off before ending the action.
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}





/// An action where the actor moves to the closest water source
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToWaterSource {
    // The movement speed of the actor.
    speed: f32,
}

/// Closest distance to a water source to be able to drink from it.
const MAX_DISTANCE: f32 = 0.1;

fn move_to_water_source_action_system(
    time: Res<Time>,
    // Find all water sources
    waters: Query<&Position, With<WaterSource>>,
    // We use Without to make disjoint queries.
    mut positions: Query<(&mut Position, &mut Transform), Without<WaterSource>>,
    // A query on all current MoveToWaterSource actions.
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToWaterSource, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                info!("Let's go find some water!");
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                let (mut actor_position, mut transform) 
                    = positions.get_mut(actor.0).expect("actor has no position");

                trace!("Actor position: {:?}", actor_position.position);

                // Look up the water source closest to them.
                let closest_water_source = find_closest_water_source(&waters, &actor_position);

                // Find how far we are from it.
                let delta = closest_water_source.position - actor_position.position;

                let distance = delta.length();

                trace!("Distance: {}", distance);

                if distance > MAX_DISTANCE {
                    // We're still too far, take a step toward it!

                    trace!("Stepping closer.");

                    // How far can we travel during this frame?
                    let step_size = time.delta_secs() * move_to.speed;
                    // Travel towards the water-source position, but make sure to not overstep it.
                    let step = delta.normalize() * step_size.min(distance);

                    // Move the actor.
                    actor_position.position += step;
                    transform.translation = Vec3::new(  actor_position.position.x, 
                                                        transform.translation.y, 
                                                        actor_position.position.y);
                } else {
                    // We're within the required distance! We can declare success.

                    info!("We got there!");

                    // The action will be cleaned up automatically.
                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                // Always treat cancellations, or we might keep doing this forever!
                // You don't need to terminate immediately, by the way, this is only a flag that
                // the cancellation has been requested. If the actor is balancing on a tightrope,
                // for instance, you may let them walk off before ending the action.
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

/// A utility function that finds the closest water source to the actor.
fn find_closest_water_source(
    waters: &Query<&Position, With<WaterSource>>,
    actor_position: &Position,
) -> Position {
    *(waters
        .iter()
        .min_by(|a, b| {
            let da = (a.position - actor_position.position).length_squared();
            let db = (b.position - actor_position.position).length_squared();
            da.partial_cmp(&db).unwrap()
        })
        .expect("no water sources"))
}

/// A simple action: the actor's thirst shall decrease, but only if they are near a water source.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Drink {
    per_second: f32,
}

fn drink_action_system(
    time: Res<Time>,
    mut thirsts: Query<(&Position, &mut Thirst), Without<WaterSource>>,
    waters: Query<&Position, With<WaterSource>>,
    mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (Actor(actor), mut state, drink, span) in &mut query {
        let _guard = span.span().enter();

        // Look up the actor's position and thirst from the Actor component in the action entity.
        let (actor_position, mut thirst) = thirsts.get_mut(*actor).expect("actor has no thirst");

        match *state {
            ActionState::Requested => {
                // We'll start drinking as soon as we're requested to do so.
                info!("Drinking the water.");
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the closest water source.
                // Note that there is no explicit passing of a selected water source from the GoToWaterSource action,
                // so we look it up again. Note that this decouples the actions from each other,
                // so if the actor is already close to a water source, the GoToWaterSource action
                // will not be necessary (though it will not harm either).
                //
                // Essentially, being close to a water source would be a precondition for the Drink action.
                // How this precondition was fulfilled is not this code's concern.
                let closest_water_source = find_closest_water_source(&waters, actor_position);

                // Find how far we are from it.
                let distance = (closest_water_source.position - actor_position.position).length();

                // Are we close enough?
                if distance < MAX_DISTANCE {
                    trace!("Drinking!");

                    // Start reducing the thirst. Alternatively, you could send out some kind of
                    // DrinkFromSource event that indirectly decreases thirst.
                     thirst.thirst -= drink.per_second * time.delta_secs();

                    // // Once we hit 0 thirst, we stop drinking and report success.
                    if thirst.thirst <= 0.0 {
                        thirst.thirst = 0.0;
                        *state = ActionState::Success;
                    }
                    // thirst.thirst = 0.0;
                    // *state = ActionState::Success;
                    
                } else {
                    // The actor was told to drink, but they can't drink when they're so far away!
                    // The action doesn't know how to deal with this case, it's the overarching system's
                    // to fulfill the precondition.
                    debug!("We're too far away!");
                    *state = ActionState::Failure;
                }
            }
            // All Actions should make sure to handle cancellations!
            // Drinking is not a complicated action, so we can just interrupt it immediately.
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

// Scorers are the same as in the thirst example.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Thirsty;

pub fn thirsty_scorer_system(
    thirsts: Query<&Thirst>,
    mut query: Query<(&Actor, &mut Score), With<Thirsty>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(thirst) = thirsts.get(*actor) {
            score.set(thirst.thirst / 100.);
            //info!("Checking Score: Thirst = {}", score.get());
        }
    }
}


// Scorers are the same as in the thirst example.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct WanderScorer;

pub fn wander_scorer_system(
    mut query: Query<(&Actor, &mut Score), With<WanderScorer>>,
) {
    for (Actor(actor), mut score) in &mut query {
        score.set(0.9);
    }
}






