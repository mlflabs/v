use bevy::prelude::*;
pub mod voxel_map;
use bevy_voxel_world::prelude::VoxelWorldPlugin;
pub use voxel_map::*;

pub mod npc;
pub use npc::*;

pub struct VoxelsPlugins;

impl Plugin for VoxelsPlugins {
    fn build(&self, app: &mut App) {
        app
            //.add_resource(MyMainWorld)
            .add_plugins(VoxelWorldPlugin::with_config(MyMainWorld))
            .add_systems(Startup, (setup, create_voxel_scene, npc_setup).chain());

    }
}






fn setup (mut cmd: Commands,){
        // light
        cmd.spawn((
            PointLight {
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0),
        ));
    
}




