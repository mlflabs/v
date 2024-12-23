use bevy::prelude::*;

use crate::{Action, MovementAction, RestScorerTag, Score, ThinkerTag, WanderScorerTag};




pub fn npc_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn((
        Name::new("Player"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 0))),
        Transform::from_xyz(2.5, 0.5, 0.5),

        ThinkerTag,
        Score::default(),
        Action::default(),
        //scorers
        WanderScorerTag,
        RestScorerTag,

        //common actions
        MovementAction::default(),

    ));
}
