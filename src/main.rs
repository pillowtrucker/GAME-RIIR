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
    render::camera::Viewport,
    window::{PresentMode, WindowMode, WindowResolution, WindowTheme},
};
use bevy_ecs_ldtk::{
    app::LdtkEntityAppExt, LdtkEntity, LdtkPlugin, LdtkWorldBundle, LevelSelection,
};
//use space_editor::{simple_editor_setup, SpaceEditorPlugin};
fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "I am a window!".into(),
                            resolution: WindowResolution::new(1920.0, 1080.0)
                                .with_scale_factor_override(1.0),
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
        .register_ldtk_entity::<PlayerBundle>("The_player")
        .run();
}
#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera2d = Camera2dBundle::default();
    //    println!("{:?}", camera.transform);
    //    println!("{:?}", camera.global_transform);
    //    bevy::log::debug!("{:?}", camera.transform);
    //    bevy::log::debug!("{:?}", camera.global_transform);
    let mut camera3d = Camera3dBundle::default();
    //    camera2d.projection.scale = 0.5;
    //camera2d.transform.translation.x += 1920.0 / 6.0 + 40.0;
    //camera2d.transform.translation.y += 1080.0 / 4.0; // += 720.0; // / 4.0;

    //    camera2d.camera.physical_target_size();

    camera2d.camera.order = 1;
    camera2d.transform.look_at(Vec3::ZERO, Vec3::Y);
    camera2d.camera.viewport = Some(Viewport {
        physical_position: UVec2 { x: 0, y: 0 },
        physical_size: UVec2 { x: 1920, y: 1080 },
        ..default()
    });
    bevy::log::info!("{:?}", camera2d.camera.logical_target_size());
    bevy::log::info!("{:?}", camera2d.camera.physical_target_size());
    bevy::log::info!("{:?}", camera3d.camera.logical_target_size());
    bevy::log::info!("{:?}", camera3d.camera.physical_target_size());
    commands.spawn(camera2d);
    commands.spawn(camera3d);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/Therac2D.ldtk"),
        ..default()
    });
}
