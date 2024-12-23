use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use std::sync::Arc;




pub const SNOWY_BRICK: u8 = 0;
pub const FULL_BRICK: u8 = 1;
pub const GRASS: u8 = 2;
pub const GROUND: u8 = 3;

#[derive(Resource, Clone, Default)]
pub struct MyMainWorld;


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





pub fn create_voxel_scene(mut voxel_world: VoxelWorld<MyMainWorld>) {
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