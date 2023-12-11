#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    unused_imports,
    non_snake_case,
    unused_mut,
    unused_variables
)]

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::query,
    prelude::*,
    render::camera::{RenderTarget, Viewport},
    window::{
        self, PresentMode, PrimaryWindow, WindowMode, WindowResized, WindowResolution, WindowTheme,
    },
    winit::WinitWindows,
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
                            //position: WindowPosition::At(IVec2 { x: 0, y: 0 }),
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
        .add_systems(Startup, (setup, query_prim_window))
        .add_systems(
            Update,
            (system, update_config, set_camera_viewports, camera_follow),
        )
        //        .add_systems(Startup, simple_editor_setup)
        .insert_resource(LevelSelection::Identifier("Level_0".to_owned()))
        .register_ldtk_entity::<PlayerBundle>("The_player")
        .run();
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

fn query_prim_window(
    q_primary: Query<Entity, With<PrimaryWindow>>,
    mut q_different: Query<&mut Window, With<PrimaryWindow>>,
    mut windows: NonSend<WinitWindows>,
) {
    let mut window = q_different.single_mut();
    let mut raw_window = windows.get_window(q_primary.single()).unwrap();
    let sf = raw_window.scale_factor();
    let isz = raw_window.inner_size();
    let osz = raw_window.outer_size();
    bevy::log::info!("{:?}", window);
    let mut monitor = raw_window.current_monitor().unwrap();
    //let mut primary_monitor = raw_window.primary_monitor();

    bevy::log::info!(
        "current monitor size {:?} position {:?} scale_factor {:?}",
        monitor.size(),
        monitor.position(),
        monitor.scale_factor()
    );
    bevy::log::info!(
        "primary monitor size {:?} position {:?} scale_factor {:?}",
        monitor.size(),
        monitor.position(),
        monitor.scale_factor()
    );
    bevy::log::info!("Primary window scale factor {sf:?} inner size {isz:?} outer size {osz:?}");
}
#[derive(Component)]
struct MyCamera2D;
#[derive(Component)]
struct MyCamera3D;
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_primary: Query<Entity, With<PrimaryWindow>>,
    mut q_different: Query<&mut Window, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let mut camera2d = Camera2dBundle::default();
    //    println!("{:?}", camera.transform);
    //    println!("{:?}", camera.global_transform);
    //    bevy::log::debug!("{:?}", camera.transform);
    //    bevy::log::debug!("{:?}", camera.global_transform);
    let mut camera3d = Camera3dBundle::default();
    camera2d.projection.scale = 0.5;
    /*

    camera2d.transform.translation.x += 1920.0 / 6.0 + 40.0;
    camera2d.transform.translation.y += 1080.0 / 4.0; // += 720.0; // / 4.0;
    */

    let mut window = q_different.single_mut();
    //    camera2d.camera.physical_target_size();
    let mut raw_window = windows.get_window(q_primary.single()).unwrap();
    camera2d.camera.target = RenderTarget::Window(window::WindowRef::Primary);
    camera2d.camera.order = 1;
    //    let wat = camera2d.camera.target;

    //    bevy::log::info!("2d camera default render target {:?}", wat);
    /*    camera2d.transform.look_at(
        Vec3 {
            x: window.resolution.physical_width() as f32 / 2.0,
            y: window.resolution.physical_height() as f32 / 2.0,
            z: 0.0,
        },
        Vec3::Y,
    );
    */
    camera2d.transform.translation.x += window.resolution.physical_width() as f32 / 16.0;
    camera2d.transform.translation.y += window.resolution.physical_height() as f32 / 16.0;
    camera2d.camera.viewport = Some(Viewport {
        physical_position: UVec2 { x: 0, y: 1480 },
        physical_size: UVec2 {
            x: window.resolution.physical_width() / 4,
            y: window.resolution.physical_height() / 4,
        },
        ..default()
    });

    bevy::log::info!("{:?}", camera2d.camera.logical_target_size());
    bevy::log::info!("{:?}", camera2d.camera.physical_target_size());
    bevy::log::info!("{:?}", camera3d.camera.logical_target_size());
    bevy::log::info!("{:?}", camera3d.camera.physical_target_size());

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
    mut camera3d: Query<&mut Camera, With<MyCamera3D>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let mut camera2d = camera2d.single_mut();
        camera2d.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2 {
                x: window.resolution.physical_width() / 4,
                y: window.resolution.physical_height() / 4,
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
                x: window.resolution.physical_width() / 4,
                y: window.resolution.physical_height() / 4,
            },
            ..default()
        });
    }

    /*
    let mut right_camera = right_camera.single_mut();
    right_camera.viewport = Some(Viewport {
        physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
        physical_size: UVec2::new(
            window.resolution.physical_width() / 2,
            window.resolution.physical_height(),
        ),
        ..default()
    });
    */
}
fn system(mut gizmos: Gizmos, time: Res<Time>) {
    let sin = time.elapsed_seconds().sin() * 50.;
    gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);

    // Triangle
    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., Color::BLUE),
        (Vec2::new(-255., -155.), Color::RED),
        (Vec2::new(255., -155.), Color::GREEN),
        (Vec2::Y * 300., Color::BLUE),
    ]);

    gizmos.rect_2d(
        Vec2::ZERO,
        time.elapsed_seconds() / 3.,
        Vec2::splat(300.),
        Color::BLACK,
    );

    // The circles have 32 line-segments by default.
    gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
    // You may want to increase this for larger circles.
    gizmos.circle_2d(Vec2::ZERO, 300., Color::NAVY).segments(64);

    // Arcs default amount of segments is linearly interpolated between
    // 1 and 32, using the arc length as scalar.
    gizmos.arc_2d(
        Vec2::ZERO,
        sin / 10.,
        std::f32::consts::PI / 2.,
        350.,
        Color::ORANGE_RED,
    );
}

fn update_config(mut config: ResMut<GizmoConfig>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    if keyboard.pressed(KeyCode::Right) {
        config.line_width += 5. * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        config.line_width -= 5. * time.delta_seconds();
    }
}
fn camera_follow(
    players: Query<(&Player, &mut Transform), (Without<MyCamera2D>)>,
    mut cameras: Query<&mut Transform, (With<MyCamera2D>, Without<MyCamera3D>)>,
) {
    for (player, player_transform) in &players {
        let pos = player_transform.translation;

        for mut transform in &mut cameras {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}
