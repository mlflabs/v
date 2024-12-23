use bevy::prelude::*;



use bevy::reflect::TypePath;
use bevy_common_assets::json::JsonAssetPlugin;


#[derive(serde::Deserialize, Asset, TypePath)]
struct Level {
    positions: Vec<[f32; 3]>,
}