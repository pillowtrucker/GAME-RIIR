#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    non_snake_case,
    unused_parens
)]

mod game_riir;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::camera::Viewport,
    window::{
        PresentMode, PrimaryWindow, WindowMode, WindowResized, WindowResolution, WindowTheme,
    },
};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_egui::{
    egui::{self, Color32, Frame, Pos2, Visuals},
    EguiContexts, EguiPlugin,
};
use game_riir::data::*;
//use space_editor::{simple_editor_setup, SpaceEditorPlugin};
fn main() {
    App::new()
        .add_plugins((
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
            //            SpaceEditorPlugin::default(),
        ))
        .init_resource::<UiStateStoryOutput>()
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup,))
        .add_systems(
            Update,
            (
                set_camera_viewports,
                camera_follow,
                hint_color,
                egui_fps,
                ui_story_output,
                fps_counter_showhide,
                fps_text_update_system,
            ),
        )
        //        .add_systems(Startup, simple_editor_setup)
        .insert_resource(LevelSelection::Identifier("Level_0".to_owned()))
        .register_ldtk_entity::<PlayerBundle>("ThePlayer")
        .register_ldtk_entity::<RatBundle>("Rat")
        .register_ldtk_entity::<BatBundle>("Bat")
        .register_ldtk_entity::<TravelPointBundle>("TravelPoint")
        .register_ldtk_entity::<TranslucentShieldingBundle>("TranslucentShielding")
        .register_ldtk_entity::<TheracLinacBundle>("TheracLinac")
        .register_ldtk_entity::<TheracVTBundle>("TheracVT")
        .register_ldtk_entity::<TheracPDP11Bundle>("TheracPDP11")
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
const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn setup(
    mut uistate: ResMut<UiStateStoryOutput>,
    //    mut contexts: EguiContexts,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 0,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    let mut uicamera2d = Camera2dBundle::default();
    uicamera2d.camera.order = 3;
    uicamera2d.camera_2d.clear_color = ClearColorConfig::None;
    uicamera2d.transform.translation.x = 1920. * 2.;
    uicamera2d.transform.translation.y = 1080. * 2.;

    let mut camera2d = Camera2dBundle::default();
    //    let mut uicamera2d = Camera2dBundle::default();
    //    uicamera2d.camera.order = 2;
    let mut camera3d = Camera3dBundle::default();
    camera2d.camera_2d.clear_color = ClearColorConfig::None;
    camera2d.projection.scale = 0.5;

    camera2d.camera.order = 2;
    camera3d.camera.order = 1;
    //    uicamera2d.projection.scale = 2.;
    //    let uicamhandle = commands.spawn((uicamera2d, UICamera2D)).id();
    let c2dent = commands.spawn((camera2d, MyCamera2D)).id();
    commands
        .entity(c2dent)
        .insert(UiCameraConfig { show_ui: false });
    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    camera3d.transform = camera_transform;
    //    let uic2dent = commands.spawn((uicamera2d, UICamera2D)).id();
    //    let c3dent = commands.spawn((camera3d, MyCamera3D)).id();
    commands.spawn((uicamera2d, UICamera2D));
    commands.spawn((camera3d, MyCamera3D));
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/Therac2D.ldtk"),
        ..default()
    });
    /*
    let ctx = contexts.ctx_mut();
    ctx.set_visuals(Visuals {
        panel_fill: Color32::TRANSPARENT,
        window_fill: Color32::TRANSPARENT,
        extreme_bg_color: Color32::TRANSPARENT,
        code_bg_color: Color32::TRANSPARENT,
        faint_bg_color: Color32::TRANSPARENT,
        ..default()
    });
    */
    uistate.text_inhalt.push_str("ooga booga");
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                transform: Transform {
                    translation: Vec3::ZERO,
                    rotation: Quat::IDENTITY,
                    scale: Vec3 {
                        x: 1.,
                        y: 2.,
                        z: 1.,
                    },
                },
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.0)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Auto,
                    top: Val::Percent(25.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Percent(25.),
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
    //    commands.entity(uic2dent).push_children(&[root]);
    //    commands.entity(c3dent).push_children(&[root]);
}

fn set_camera_viewports(
    windows: Query<&Window>,
    //    mut fpsdings: Query<&mut Style, With<FpsRoot>>,
    mut resize_events: EventReader<WindowResized>,
    mut window_move_events: EventReader<WindowMoved>,
    mut uicamera2d: Query<
        &mut Camera,
        (With<UICamera2D>, Without<MyCamera2D>, Without<MyCamera3D>),
    >,
    mut camera2d: Query<&mut Camera, (With<MyCamera2D>, Without<MyCamera3D>, Without<UICamera2D>)>,
    mut camera3d: Query<&mut Camera, (With<MyCamera3D>, Without<MyCamera2D>, Without<UICamera2D>)>,
) {
    /*
    We need to dynamically resize the camera's viewports whenever the window size changes
    A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    this is actually the only place where it makes sense to set this since the source coordinates set in setup could be wrong
    */
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let mut camera2d = camera2d.single_mut();
        let mut camera3d = camera3d.single_mut();
        let mut uicamera2d = uicamera2d.single_mut();

        camera3d.viewport = Some(Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
            physical_size: UVec2 {
                x: window.resolution.physical_width() / 2,
                y: window.resolution.physical_height(),
            },
            ..default()
        });
        uicamera2d.viewport = Some(Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
            physical_size: UVec2 {
                x: window.resolution.physical_width() / 2,
                y: window.resolution.physical_height(),
            },
            ..default()
        });
        camera2d.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2 {
                x: window.resolution.physical_width() / 2,
                y: window.resolution.physical_height() / 2,
            },
            ..default()
        });
    }
    for move_event in window_move_events.read() {
        let window = windows.get(move_event.entity).unwrap();
        let mut camera2d = camera2d.single_mut();

        let mut camera3d = camera3d.single_mut();
        let mut uicamera2d = uicamera2d.single_mut();
        camera3d.viewport = Some(Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
            physical_size: UVec2 {
                x: window.resolution.physical_width() / 2,
                y: window.resolution.physical_height(),
            },
            ..default()
        });
        uicamera2d.viewport = Some(Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
            physical_size: UVec2 {
                x: window.resolution.physical_width() / 2,
                y: window.resolution.physical_height(),
            },
            ..default()
        });
        camera2d.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2 {
                x: window.resolution.physical_width() / 2,
                y: window.resolution.physical_height() / 2,
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
        } else {
            break;
        }
    }
    for (_item, mut item_sprite) in &mut items {
        if (item_sprite.color != Color::CYAN) {
            let res = item_sprite.color.set(Box::new(Color::CYAN));
            bevy::log::info!("{:?}", res);
        } else {
            break;
        }
    }
    /*
    for mut other_sprite in &mut rest {
        bevy::log::info!("{:?}", other_sprite);
        other_sprite.color = Color::BLACK;
    }
    */
}
fn ui_story_output(
    mut uistate: ResMut<UiStateStoryOutput>,
    pwindow: Query<&Window, With<PrimaryWindow>>,
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();
    let w = pwindow.single();
    let pwindow_h = w.resolution.physical_height();
    let pwindow_w = w.resolution.physical_width();
    egui::Window::new("StoryOutput")
        .title_bar(false)
        .interactable(false)
        .frame(Frame::none())
        .fixed_pos(Pos2 {
            x: 0.,
            y: pwindow_h as f32 / 2.0,
        })
        .fixed_size(egui::Vec2 {
            x: pwindow_w as f32 / 2.0,
            y: pwindow_h as f32 / 2.0,
        })
        .show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut uistate.text_inhalt),
            )
        });
}
#[derive(Default, Resource)]
struct UiStateStoryOutput {
    text_inhalt: String,
}

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText;

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

/// Toggle the FPS counter when pressing F12
fn fps_counter_showhide(mut q: Query<&mut Visibility, With<FpsRoot>>, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
