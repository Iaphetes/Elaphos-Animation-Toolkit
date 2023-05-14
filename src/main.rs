//! Illustrates bloom post-processing in 2d.

use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::{Label, *},
    sprite::MaterialMesh2dBundle,
    text::Text,
};
use core::ops::Add;
#[derive(Component)]
struct Fade {
    pub speed: f32,
    pub fade_amount: f32,
}
#[derive(Component, Eq, PartialEq)]
struct ObjectLabel(String);

struct TextFadeEvent {
    speed: f32,
    label: ObjectLabel,
}
enum AnimationEvent {
    TextFade(TextFadeEvent),
}
#[derive(Resource)]
struct Counter(u32);
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .add_plugins(DefaultPlugins)
        .insert_resource(Counter(0))
        .add_event::<AnimationEvent>()
        .add_startup_system(setup)
        .add_system(text_fade)
        .add_system(text_fade_init)
        .add_system(animation_sequence)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
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
        font: font.clone(),
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
            text: Text::from_section("S", text_style.clone()).with_alignment(text_alignment),
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
                *counter = Counter(counter.0 + 1);
            }
            Counter(1) => {
                animation_events.send(AnimationEvent::TextFade(TextFadeEvent {
                    speed: 1.0,
                    label: ObjectLabel("ECS-C".to_string()),
                }));
            }
            Counter(_) => {}
        }
    }
}
fn text_fade_init(
    mut commands: Commands,
    mut animation_events: EventReader<AnimationEvent>,
    mut texts: Query<(Entity, &mut Text, &ObjectLabel)>,
) {
    for animation_event in &mut animation_events {
        if let AnimationEvent::TextFade(text_fade_event) = animation_event {
            for (entity, text, object_label) in &texts {
                if object_label == &text_fade_event.label {
                    let fade_amount: f32 = text.sections[0].style.color.a();
                    commands.entity(entity).insert(Fade {
                        speed: text_fade_event.speed,
                        fade_amount,
                    });
                }
            }
        }
    }
}
fn text_fade(
    mut commands: Commands,
    time: Res<Time>,
    mut texts: Query<(Entity, &mut Text, &mut Fade)>,
) {
    for (entity, mut text, mut fade) in texts.iter_mut() {
        for section in &mut text.sections {
            section.style.color = section.style.color.with_a(
                section.style.color.a() - fade.fade_amount * time.delta_seconds() / fade.speed,
            );
            if section.style.color.a() <= 0.0 {
                section.style.color = section.style.color.with_a(0.0);
                commands.entity(entity).remove::<Fade>();
            }
        }
    }
}
