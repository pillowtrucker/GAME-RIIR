#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    non_snake_case,
    unused_parens
)]

mod game_riir;

use std::process::exit;

use bevy::{
    asset::{
        io::{AssetReaderError, Reader},
        AssetLoader, AsyncReadExt, LoadContext, LoadState,
    },
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
    egui::{self, Color32, Frame, Pos2, RichText, Visuals},
    EguiContexts, EguiPlugin,
};
use game_riir::data::*;

use rand::Rng;
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
        //        .init_resource::<MyTextCollection>()
        .init_resource::<UiStateStoryOutput>()
        .init_asset_loader::<MyTextLoader>()
        .init_asset::<MyText>()
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
            ui.label(RichText::new(format!("{:.3}", value)))
        });
    }
}
const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);
#[derive(Default)]
struct MyTextLoader;

impl AssetLoader for MyTextLoader {
    type Asset = MyText;
    type Settings = ();
    type Error = AssetReaderError;

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut the_body = String::new();

            reader.read_to_string(&mut the_body).await?;
            Ok(MyText {
                title: the_body.lines().collect::<Vec<_>>()[0].to_owned(),
                body: the_body,
            })
        })
    }
}

#[derive(Asset, Default, Resource, TypePath, Clone)]
struct MyText {
    title: String,
    body: String,
}

fn setup(
    mut contexts: EguiContexts,
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

    let ctx = contexts.ctx_mut();
    ctx.set_visuals(Visuals {
        panel_fill: Color32::TRANSPARENT,
        window_fill: Color32::TRANSPARENT,
        extreme_bg_color: Color32::TRANSPARENT,
        code_bg_color: Color32::TRANSPARENT,
        faint_bg_color: Color32::TRANSPARENT,
        ..default()
    });

    let hooga: Handle<MyText> = asset_server.load("texts/PARADISE_LOST.txt");
    commands.spawn(hooga);
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
    asset_server: Res<AssetServer>,
    mytexts: Res<Assets<MyText>>,
) {
    let ctx = contexts.ctx_mut();
    let pl_handle = asset_server.get_handle("texts/PARADISE_LOST.txt");
    if let Some(pl_handle) = pl_handle {
        if uistate.text_inhalt.is_empty() {
            let fuck = asset_server.load_state(&pl_handle);
            match fuck {
                LoadState::Failed => exit(1),
                LoadState::Loaded => {
                    let mut rng = rand::thread_rng();
                    let pl = &mytexts.get(pl_handle).unwrap().body;
                    let good_number = rng.gen_range(0..pl.lines().count());
                    let random_fragment =
                        pl.lines().collect::<Vec<_>>()[good_number..good_number + 66].to_owned();
                    uistate.text_inhalt.push_str(&random_fragment.join("\n"));
                }
                _ => {}
            }
        }
    }
    let w = pwindow.single();
    let pwindow_h = w.resolution.physical_height();
    let pwindow_w = w.resolution.physical_width();
    egui::Window::new("StoryOutput")
        .title_bar(false)
        .interactable(false)
        .frame(Frame::none())
        .fixed_pos(Pos2 {
            x: pwindow_w as f32 / 2.0,
            y: pwindow_h as f32 / 2.0,
        })
        .fixed_size(egui::Vec2 {
            x: pwindow_w as f32 / 2.0,
            y: pwindow_h as f32,
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
