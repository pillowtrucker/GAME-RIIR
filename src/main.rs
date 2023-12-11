#![allow(clippy::too_many_arguments, clippy::type_complexity, non_snake_case)]

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::camera::Viewport,
    window::{PresentMode, WindowMode, WindowResized, WindowResolution, WindowTheme},
};
use bevy_ecs_ldtk::{
    app::LdtkEntityAppExt, GridCoords, LdtkEntity, LdtkPlugin, LdtkWorldBundle, LevelSelection,
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
        .add_systems(Update, (set_camera_viewports, camera_follow))
        //        .add_systems(Startup, simple_editor_setup)
        .insert_resource(LevelSelection::Identifier("Level_0".to_owned()))
        .register_ldtk_entity::<PlayerBundle>("ThePlayer")
        .register_ldtk_entity::<RatBundle>("Rat")
        .register_ldtk_entity::<BatBundle>("Bat")
        .register_ldtk_entity::<TheBusStopBundle>("TheBusStop")
        .register_ldtk_entity::<ShowerBundle>("Shower")
        .register_ldtk_entity::<ToiletBundle>("Toilet")
        .register_ldtk_entity::<SoiledMattressBundle>("SoiledMattress")
        .register_ldtk_entity::<CoinDoorBundle>("CoinDoor")
        .register_ldtk_entity::<BrokenCoinDoorBundle>("BrokenCoinDoor")
        .run();
}

#[derive(Default, Bundle, LdtkEntity)]
struct TheBusStopBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct ToiletBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
#[derive(Default, Bundle, LdtkEntity)]
struct ShowerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct RatBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
#[derive(Default, Bundle, LdtkEntity)]
struct BatBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
#[derive(Default, Bundle, LdtkEntity)]
struct CoinDoorBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct BrokenCoinDoorBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct SoiledMattressBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Component)]
struct Player;
#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component)]
struct MyCamera2D;
#[derive(Component)]
struct MyCamera3D;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera2d = Camera2dBundle::default();

    let camera3d = Camera3dBundle::default();
    camera2d.projection.scale = 0.25;

    camera2d.camera.order = 1;

    commands.spawn((camera2d, MyCamera2D));
    commands.spawn((camera3d, MyCamera3D));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/Therac2D.ldtk"),
        ..default()
    });
}
fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut window_move_events: EventReader<WindowMoved>,
    mut camera2d: Query<&mut Camera, (With<MyCamera2D>, Without<MyCamera3D>)>,
) {
    /*
    We need to dynamically resize the camera's viewports whenever the window size changes
    A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    this is actually the only place where it makes sense to set this since the source coordinates set in setup could be wrong
    */
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let mut camera2d = camera2d.single_mut();
        camera2d.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2 {
                x: window.resolution.physical_width() / 3,
                y: window.resolution.physical_height() / 3,
            },
            ..default()
        });
    }
    for move_event in window_move_events.read() {
        let window = windows.get(move_event.entity).unwrap();
        let mut camera2d = camera2d.single_mut();
        camera2d.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2 {
                x: window.resolution.physical_width() / 3,
                y: window.resolution.physical_height() / 3,
            },
            ..default()
        });
    }
}

fn camera_follow(
    players: Query<(&Player, &mut Transform), (Without<MyCamera2D>, Without<MyCamera3D>)>,
    mut cameras: Query<&mut Transform, (With<MyCamera2D>, Without<MyCamera3D>)>,
) {
    for (_player, player_transform) in &players {
        let pos = player_transform.translation;

        for mut transform in &mut cameras {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}
