#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    non_snake_case,
    unused_parens
)]

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::camera::Viewport,
    window::{PresentMode, WindowMode, WindowResized, WindowResolution, WindowTheme},
};
use bevy_ecs_ldtk::{
    app::LdtkEntityAppExt, GridCoords, LdtkEntity, LdtkPlugin, LdtkWorldBundle, LevelSelection,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
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
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup))
        .add_systems(
            Update,
            (set_camera_viewports, camera_follow, hint_color, egui_fps),
        )
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
fn egui_fps(mut contexts: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    if let Some(value) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        egui::Window::new("FPS").show(contexts.ctx_mut(), |ui| {
            ui.label(format!("{:.3}", value));
        });
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct TheBusStopBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct ToiletBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
#[derive(Default, Bundle, LdtkEntity)]
struct ShowerBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct RatBundle {
    actor: Actor,
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
#[derive(Default, Bundle, LdtkEntity)]
struct BatBundle {
    actor: Actor,
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
#[derive(Default, Bundle, LdtkEntity)]
struct CoinDoorBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct BrokenCoinDoorBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct SoiledMattressBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Component)]
struct Interactive;
#[derive(Default, Component)]
struct Actor;

#[derive(Default, Component)]
struct Player;
#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    actor: Actor,
    interactive: Interactive,
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component)]
struct UICamera2D;

#[derive(Component)]
struct MyCamera2D;
#[derive(Component)]
struct MyCamera3D;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera2d = Camera2dBundle::default();
    //    let mut uicamera2d = Camera2dBundle::default();
    //    uicamera2d.camera.order = 2;
    let camera3d = Camera3dBundle::default();
    camera2d.projection.scale = 0.5;

    camera2d.camera.order = 1;
    //    let uicamhandle = commands.spawn((uicamera2d, UICamera2D)).id();
    commands.spawn((camera2d, MyCamera2D));
    commands.spawn((camera3d, MyCamera3D));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/Therac2D.ldtk"),
        ..default()
    });
}

fn set_camera_viewports(
    windows: Query<&Window>,
    //    mut fpsdings: Query<&mut Style, With<FpsRoot>>,
    mut resize_events: EventReader<WindowResized>,
    mut window_move_events: EventReader<WindowMoved>,
    mut camera2d: Query<&mut Camera, (With<MyCamera2D>, Without<MyCamera3D>, Without<UICamera2D>)>,
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

fn hint_color(
    mut actors: Query<(&Actor, &mut TextureAtlasSprite), (With<Actor>)>,
    mut items: Query<(&Interactive, &mut TextureAtlasSprite), (Without<Actor>)>,
    /* this matched background sprite only..
    mut rest: Query<(&mut Sprite), (Without<Actor>, Without<Interactive>)>,
    */
    /* this matched nothing
    mut rest: Query<(&mut TextureAtlasSprite), (Without<Actor>, Without<Interactive>)>,
    */
) {
    for (_actor, mut actor_sprite) in &mut actors {
        if (actor_sprite.color != Color::CRIMSON) {
            let res = actor_sprite.color.set(Box::new(Color::CRIMSON));
            bevy::log::info!("{:?}", res);
        }
    }
    for (_item, mut item_sprite) in &mut items {
        if (item_sprite.color != Color::CYAN) {
            let res = item_sprite.color.set(Box::new(Color::CYAN));
            bevy::log::info!("{:?}", res);
        }
    }
    /*
    for mut other_sprite in &mut rest {
        bevy::log::info!("{:?}", other_sprite);
        other_sprite.color = Color::BLACK;
    }
    */
}
