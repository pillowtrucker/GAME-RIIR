// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowMode, WindowTheme},
};
use space_editor::{simple_editor_setup, SpaceEditorPlugin};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
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
            }),
            //            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            SpaceEditorPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, simple_editor_setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ducky.png"),
        ..Default::default()
    });
}
