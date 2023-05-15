use bevy::{prelude::*, text::Text};

use crate::animation::{AnimationEvent, ObjectLabel};
#[derive(Component)]
struct Fade {
    pub speed: f32,
    pub fade_amount: f32,
}
pub struct TextFadeEvent {
    pub speed: f32,
    pub label: ObjectLabel,
}
pub struct FadePlugin;

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvent>()
            .add_system(text_fade_init)
            .add_system(text_fade);
    }
}
fn text_fade_init(
    mut commands: Commands,
    mut animation_events: EventReader<AnimationEvent>,
    texts: Query<(Entity, &mut Text, &ObjectLabel)>,
) {
    for animation_event in &mut animation_events {
        if let AnimationEvent::TextFade(text_fade_event) = animation_event {
            for (entity, text, object_label) in &texts {
                if object_label == &text_fade_event.label {
                    let mut fade_amount: f32 = text.sections[0].style.color.a();
                    if fade_amount <= 0.0 {
                        fade_amount = 1.0;
                    }
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
    for (entity, mut text, fade) in texts.iter_mut() {
        for section in &mut text.sections {
            println!("alpha {}", section.style.color.a());
            section.style.color = section.style.color.with_a(
                section.style.color.a() - fade.fade_amount * time.delta_seconds() / fade.speed,
            );
            if fade.speed > 0.0 && section.style.color.a() <= 0.0 {
                section.style.color = section.style.color.with_a(0.0);
                commands.entity(entity).remove::<Fade>();
            } else if fade.speed < 0.0 && section.style.color.a() >= 1.0 {
                section.style.color = section.style.color.with_a(1.0);
            }
        }
    }
}
