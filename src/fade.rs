use bevy::{prelude::*, text::Text};

use crate::animation::{ElaphosAnimationEvent, ObjectLabel};
#[derive(Component)]
struct Fade {
    pub speed: f32,
    pub fade_amount: f32,
}
pub struct FadeEvent {
    pub speed: f32,
    pub label: ObjectLabel,
}
pub struct ElaphosFadePlugin;

impl Plugin for ElaphosFadePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ElaphosAnimationEvent>()
            .add_system(fade_init)
            .add_system(fade_system);
    }
}

fn fade_init(
    mut commands: Commands,
    mut animation_events: EventReader<ElaphosAnimationEvent>,
    // materials: AssetServer<StandardMaterial>,
    texts: Query<(Entity, &mut Text, &ObjectLabel)>,
    sprites: Query<(Entity, &mut Sprite, &ObjectLabel)>,
    // scenes: Query<(Entity, &mut Scene, &ObjectLabel)>,
) {
    for animation_event in &mut animation_events {
        if let ElaphosAnimationEvent::Fade(fade_event) = animation_event {
            let mut target_entity: Option<Entity> = None;
            let mut target_color: Option<Color> = None;
            for (entity, text, object_label) in &texts {
                if object_label == &fade_event.label {
                    target_entity = Some(entity);
                    target_color = Some(text.sections[0].style.color);
                }
            }
            for (entity, sprite, object_label) in &sprites {
                if object_label == &fade_event.label {
                    target_entity = Some(entity);
                    target_color = Some(sprite.color);
                }
            }
            // for (entity, scene, object_label) in &scenes {
            //     println!("{:#?}", scene);
            // }
            if let (Some(entity), Some(color)) = (target_entity, target_color) {
                let mut fade_amount: f32 = color.a();
                if fade_amount <= 0.0 {
                    fade_amount = 1.0;
                }
                commands.entity(entity).insert(Fade {
                    speed: fade_event.speed,
                    fade_amount,
                });
            }
        }
    }
}
fn fade_alpha(color: &mut Color, fade: &Fade, delta_seconds: f32) -> bool {
    *color = color.with_a(color.a() - fade.fade_amount * delta_seconds * fade.speed);
    if fade.speed > 0.0 && color.a() <= 0.0 {
        *color = color.with_a(0.0);
        true
    } else if fade.speed < 0.0 && color.a() >= 1.0 {
        *color = color.with_a(1.0);
        true
    } else {
        false
    }
}
fn fade_system(
    mut commands: Commands,
    time: Res<Time>,
    mut texts: Query<(Entity, &mut Text, &Fade)>,
    mut sprites: Query<(Entity, &mut Sprite, &Fade)>,
) {
    for (entity, mut text, fade) in texts.iter_mut() {
        let mut all_sections_finished: bool = true;
        for section in text.sections.iter_mut() {
            if !fade_alpha(&mut section.style.color, fade, time.delta_seconds()) {
                all_sections_finished = false;
            }
        }
        if all_sections_finished {
            commands.entity(entity).remove::<Fade>();
        }
    }
    for (entity, mut sprite, fade) in sprites.iter_mut() {
        if fade_alpha(&mut sprite.color, fade, time.delta_seconds()) {
            commands.entity(entity).remove::<Fade>();
        }
    }
}
