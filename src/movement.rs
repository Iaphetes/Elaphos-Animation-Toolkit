use bevy::prelude::*;

use crate::{animation::ObjectLabel, AnimationEvent};
#[derive(Component)]
struct Translate {
    waypoints: Vec<Vec3>,
    speed: f32,
    movement_type: TranslationType,
}
#[derive(Clone, Copy)]
pub enum TranslationType {
    Linear,
}
#[derive(Component)]
pub struct TranslateEvent {
    pub waypoints: Vec<Vec3>,
    pub speed: f32,
    pub movement_type: TranslationType,
    pub label: ObjectLabel,
}
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvent>()
            .add_system(translation_init)
            .add_system(perform_translation);
    }
}
fn translation_init(
    mut commands: Commands,
    mut animation_events: EventReader<AnimationEvent>,
    transforms: Query<(Entity, &ObjectLabel)>,
) {
    for animation_event in &mut animation_events {
        if let AnimationEvent::Movement(movement_event) = animation_event {
            for (entity, object_label) in &transforms {
                if object_label == &movement_event.label {
                    commands.entity(entity).insert(Translate {
                        waypoints: movement_event.waypoints.clone(),
                        speed: movement_event.speed,
                        movement_type: movement_event.movement_type,
                    });
                }
            }
        }
    }
}
fn perform_translation(
    mut commands: Commands,
    time: Res<Time>,
    mut objects: Query<(Entity, &mut Transform, &mut Translate)>,
) {
    for (entity, mut transform, movement) in objects.iter_mut() {
        match movement.movement_type {
            TranslationType::Linear => {
                let target: Vec3 = movement.waypoints[0];
                let move_amount: Vec3 = (target - transform.translation).normalize()
                    * time.delta_seconds()
                    * movement.speed;

                let new_pos: Vec3;
                if move_amount.length() > (target - transform.translation).length() {
                    new_pos = target;
                    commands.entity(entity).remove::<Translate>();
                } else {
                    new_pos = transform.translation + move_amount;
                }
                println!("Moving to {:?}", new_pos);
                transform.translation = new_pos;
            }
        }
    }
}
