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
        .add_systems(Startup, (setup, setup_fps_counter))
        .add_systems(
            Update,
            (
                set_camera_viewports,
                camera_follow,
                hint_color,
                fps_text_update_system,
                fps_counter_showhide,
            ),
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
struct MyCamera2D;
#[derive(Component)]
struct MyCamera3D;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera2d = Camera2dBundle::default();

    let camera3d = Camera3dBundle::default();
    camera2d.projection.scale = 0.5;

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
    //    mut fpsdings: Query<&mut Style, With<FpsRoot>>,
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
        /*
        let mut fps = fpsdings.single_mut();
        fps.left = Val::Percent(1.);
        fps.top = Val::Percent(1.);
        fps.position_type = PositionType::Absolute;
        */
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
        /*
        let mut fps = fpsdings.single_mut();
        fps.left = Val::Percent(1.);
        fps.top = Val::Percent(1.);
        fps.position_type = PositionType::Absolute;
        */
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
/*
fn unfuck_fps_position(mut fpsdings: Query<&mut Style, With<FpsRoot>>) {
    let mut fps = fpsdings.single_mut();
    fps.right = Val::Percent(1.);
    fps.top = Val::Percent(1.);
    fps.position_type = PositionType::Absolute;
}
*/
/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText;

fn setup_fps_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Auto,
                    top: Val::Percent(0.5),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Percent(0.5),
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_fps = commands
        .spawn((
            FpsText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

/// Toggle the FPS counter when pressing F6
fn fps_counter_showhide(mut q: Query<&mut Visibility, With<FpsRoot>>, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::F6) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
