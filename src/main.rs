//! Illustrates bloom post-processing in 2d.

use std::f32::consts::PI;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    text::Text,
};
mod animation;
mod fade;
mod movement;
use animation::{AnimationEvent, ObjectLabel};
use fade::{FadePlugin, TextFadeEvent};
use movement::{MovementPlugin, RotateEvent, RotationType, TranslateEvent, TranslationType};

#[derive(Resource)]
struct Counter(u32);
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .add_plugins(DefaultPlugins)
        .insert_resource(Counter(0))
        .add_event::<AnimationEvent>()
        .add_plugin(FadePlugin)
        .add_plugin(MovementPlugin)
        .add_startup_system(setup)
        .add_system(animation_sequence)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));
    let font = asset_server.load("./fonts/anodina/Anodina-Bold.otf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::rgb(0.0, 0.5, 0.0),
    };
    let text_alignment = TextAlignment::Center;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("E", text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ObjectLabel("ECS-E".to_string()),
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("C", text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_xyz(60.0, 0.0, 0.0),
            ..default()
        },
        ObjectLabel("ECS-C".to_string()),
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("S", text_style).with_alignment(text_alignment),
            transform: Transform::from_xyz(120.0, 0.0, 0.0),
            ..default()
        },
        ObjectLabel("ECS-S".to_string()),
    ));
}
fn animation_sequence(
    mut counter: ResMut<Counter>,
    mut animation_events: EventWriter<AnimationEvent>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match counter.as_ref() {
            Counter(0) => {
                animation_events.send(AnimationEvent::TextFade(TextFadeEvent {
                    speed: 1.0,
                    label: ObjectLabel("ECS-S".to_string()),
                }));
                animation_events.send(AnimationEvent::Translate(TranslateEvent {
                    waypoints: vec![Vec3 {
                        x: 0.0,
                        y: 60.0,
                        z: 0.0,
                    }],
                    speed: 100.0,
                    movement_type: TranslationType::LinearAbsolute,
                    label: ObjectLabel("ECS-E".to_string()),
                }));
                animation_events.send(AnimationEvent::Rotate(RotateEvent {
                    rotation_amount: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: f32::to_radians(180.0),
                    },
                    speed: 10.0,
                    rotate_type: RotationType::LinearAbsolute,
                    label: ObjectLabel("ECS-C".to_string()),
                }));
            }
            Counter(1) => {
                animation_events.send(AnimationEvent::TextFade(TextFadeEvent {
                    speed: -1.0,
                    label: ObjectLabel("ECS-S".to_string()),
                }));
                animation_events.send(AnimationEvent::Translate(TranslateEvent {
                    waypoints: vec![Vec3 {
                        x: 0.0,
                        y: 60.0,
                        z: 0.0,
                    }],
                    speed: 100.0,

                    movement_type: TranslationType::LinearAbsolute,
                    label: ObjectLabel("ECS-E".to_string()),
                }));
            }
            Counter(_) => {}
        }
        counter.0 += 1;
    }
}
