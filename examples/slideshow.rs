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
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ElaphosDefaultPlugins)
        .insert_resource(Counter(0))
        .add_event::<ElaphosAnimationEvent>()
        .add_startup_system(setup)
        .add_system(animation_sequence)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));
    let font = asset_server.load("./fonts/anodina/Anodina-Bold.otf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::rgb(0.0, 0.5, 0.0),
    };
    let text_alignment = TextAlignment::Center;
    // Spawn a text bundle with the ObjectLabel "Elaphos"
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Elaphos", text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_xyz(30.0, 0.0, 0.0),
            ..default()
        },
        ObjectLabel("Elaphos".to_string()),
    ));
    // Spawn a Sprite with a texture and the ObjectLabel "Box"
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("basics/ui_rect.png"),
            transform: Transform::from_xyz(0.0, -900.0, 0.0).with_scale(Vec3::splat(0.8)),
            ..default()
        },
        ObjectLabel("Box".to_string()),
    ));
}
fn animation_sequence(
    mut counter: ResMut<Counter>,
    mut animation_events: EventWriter<ElaphosAnimationEvent>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match counter.as_ref() {
            Counter(0) => {
                // Fade out the text
                animation_events.send(ElaphosAnimationEvent::Fade(FadeEvent {
                    speed: 10.0,
                    label: ObjectLabel("Elaphos".to_string()),
                }));
                // Move up the box into the field
                animation_events.send(ElaphosAnimationEvent::Translate(TranslateEvent {
                    waypoints: vec![Vec3 {
                        x: 0.0,
                        y: 450.0,
                        z: 0.0,
                    }],
                    speed: 600.0,
                    movement_type: TranslationType::LinearRelative,
                    label: ObjectLabel("Box".to_string()),
                }));
                // Change the background color to a dark blue
                animation_events.send(ElaphosAnimationEvent::Background(BackgroundEvent {
                    speed: 1.0,
                    color: Color::NAVY,
                }));
            }
            Counter(1) => {
                // Fade the text basck in (denoted by the minus sign in front of the speed)
                animation_events.send(ElaphosAnimationEvent::Fade(FadeEvent {
                    speed: -10.0,
                    label: ObjectLabel("Elaphos".to_string()),
                }));
            }
            Counter(2) => {
                // Fade out the box
                animation_events.send(ElaphosAnimationEvent::Fade(FadeEvent {
                    speed: 10.0,
                    label: ObjectLabel("Box".to_string()),
                }));
            }
            Counter(_) => {}
        }
        counter.0 += 1;
    }
}
