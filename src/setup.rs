use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
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
        );
        
       
        
    } 
}