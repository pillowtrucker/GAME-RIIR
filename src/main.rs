#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    unused_imports,
    non_snake_case,
    unused_mut
)]

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, WindowMode, WindowTheme},
};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};
//use space_editor::{simple_editor_setup, SpaceEditorPlugin};
fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "I am a window!".into(),
                            decorations: false,
                            mode: WindowMode::Windowed,
                            present_mode: PresentMode::AutoVsync,
                            // Tells wasm to resize the window according to the available canvas
                            fit_canvas_to_parent: true,
                            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                            prevent_default_event_handling: false,
                            window_theme: Some(WindowTheme::Dark),
                            ..default()
                        }),
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
                //            LogDiagnosticsPlugin::default(),
                FrameTimeDiagnosticsPlugin,
                LdtkPlugin,
            ), //            SpaceEditorPlugin::default(),
        )
        .add_systems(Startup, setup)
        //        .add_systems(Startup, simple_editor_setup)
        .insert_resource(LevelSelection::Identifier("Level_0".to_owned()))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    //    camera.projection.scale = 0.5;
    //    camera.transform.translation.x += 1280.0 / 4.0;
    //    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/Therac2D.ldtk"),
        ..Default::default()
    });
}
