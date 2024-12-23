use bevy::{prelude::*, remote::{http::RemoteHttpPlugin, RemotePlugin}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::reflect::TypePath;
use bevy_common_assets::json::JsonAssetPlugin;

mod setup;
use setup::*;

mod game;
use game::*;


fn main() {
    App::new()
        .add_plugins(SetupPlugin)

        .init_state::<AppState>()

        //.add_plugins(WorldInspectorPlugin::new())
        //.add_plugins(GamePlugin)
        //.add_plugins(RemotePlugin::default())
        //.add_plugins(RemoteHttpPlugin::default())

        .add_plugins(JsonAssetPlugin::<Map>::new(&[".json"]))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_level.run_if(in_state(AppState::Loading)))


        .run();
        
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = MapHandle(asset_server.load("tiled/map4x4.json"));
    commands.insert_resource(map);
    //let tree = ImageHandle(asset_server.load("tree.png"));
    //.commands.insert_resource(tree);

    //commands.spawn((Camera2d, Msaa::Off));
}

fn spawn_level(
    mut commands: Commands,
    level: Res<MapHandle>,
    //tree: Res<ImageHandle>,
    mut levels: ResMut<Assets<Map>>,
    mut state: ResMut<NextState<AppState>>,
) {
    if let Some(level) = levels.remove(level.0.id()) {
        println!("Level:::: {:?}", level);
        // for position in level.positions {
        //     commands.spawn((
        //         Sprite::from_image(tree.0.clone()),
        //         Transform::from_translation(position.into()),
        //     ));
        // }

        state.set(AppState::Level);
    }
}




#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Level,
}


#[derive(serde::Deserialize, Asset, TypePath, Debug)]
struct Map {
    pub height: usize,
    pub width: usize,
    pub tilewidth: usize,
    pub tileheight: usize,
    //pub version: f32,
    layers: Vec<Layer>,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
struct Layer {
    data: Vec<usize>,
    height: usize,
    width: usize,
    id: usize,
    //name: String,
    opacity: f32,
    visible: bool,
    x: usize,
    y: usize,
}





// #[derive(Resource)]
// struct ImageHandle(Handle<Image>);

#[derive(Resource)]
struct MapHandle(Handle<Map>);