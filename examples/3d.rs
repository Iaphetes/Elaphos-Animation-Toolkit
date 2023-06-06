use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    text::Text,
};
use elaphos::animation::{ElaphosAnimationEvent, ElaphosDefaultPlugins, ObjectLabel};
use elaphos::change_background::BackgroundEvent;
use elaphos::fade::FadeEvent;
use elaphos::movement::{RotateEvent, RotationType, TranslateEvent, TranslationType};
#[derive(Resource)]
struct Counter(u32);
#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ElaphosDefaultPlugins)
        .insert_resource(Counter(0))
        .insert_resource(Animations(Vec::new()))
        .add_event::<ElaphosAnimationEvent>()
        .add_startup_system(setup)
        .add_system(animation_sequence)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animations: ResMut<Animations>,
) {
    // 2D camera to render sprites (Rendered in the background)
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                order: 0,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));
    // 3D camera to render 3D objects (rendered in the foreground with no background)
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                order: 1,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::None,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));

    let font = asset_server.load("./fonts/jupiteroid/JupiteroidBold-9YgLj.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::rgb(0.0, 0.5, 0.0),
    };
    let text_alignment = TextAlignment::Center;
    // Spawn a text bundle with the ObjectLabel "Elaphos"
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("3d_models/Earth Hologram.gltf#Scene0"),
            transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3 {
                x: -5.0,
                y: 0.0,
                z: 0.0,
            }),
            ..default()
        },
        ObjectLabel("Earth_Hologram".to_owned()),
    ));

    commands.spawn((
        SceneBundle {
            scene: asset_server.load("3d_models/Earth Hologram.gltf#Scene0"),
            transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3 {
                x: 5.0,
                y: 0.0,
                z: 0.0,
            }),
            ..default()
        },
        ObjectLabel("Earth_Hologram_2".to_owned()),
    ));
    animations
        .0
        .push(asset_server.load("3d_models/Earth Hologram.gltf#Animation0"));
}
fn animation_sequence(
    mut counter: ResMut<Counter>,
    mut animation_events: EventWriter<ElaphosAnimationEvent>,
    keys: Res<Input<KeyCode>>,

    mut animation_player: Query<&mut AnimationPlayer>,
    mut animations: ResMut<Animations>,
    animation_elements: Query<Entity, With<ObjectLabel>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match counter.as_ref() {
            Counter(0) => {
                animation_events.send(ElaphosAnimationEvent::Fade(FadeEvent {
                    speed: 0.5,
                    label: ObjectLabel("Earth_Hologram".to_string()),
                }));
            }
            Counter(1) => {
                for (idx, mut player) in animation_player.iter_mut().enumerate() {
                    player.play(animations.0[0].clone_weak()).repeat();
                    player.set_speed(0.5);
                }

                animation_events.send(ElaphosAnimationEvent::Fade(FadeEvent {
                    speed: -0.5,
                    label: ObjectLabel("Earth_Hologram".to_string()),
                }));
            }
            Counter(2) => {}
            Counter(_) => {}
        }
        counter.0 += 1;
    }
}
