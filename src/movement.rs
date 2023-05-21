use bevy::prelude::*;

use crate::animation::{AnimationEvent, ObjectLabel};
#[derive(Component)]
struct Translate {
    waypoints: Vec<Vec3>,
    speed: f32,
    movement_type: TranslationType,
}
#[derive(Component)]
struct Rotate {
    rotation_amount: Vec3,
    speed: f32,
    rotation_type: RotationType,
}
#[derive(Clone, Copy)]
pub enum TranslationType {
    LinearAbsolute,
    LinearRelative,
}

#[derive(Clone, Copy)]
pub enum RotationType {
    LinearAbsolute,
    LinearRelative,
}
#[derive(Component)]
pub struct RotateEvent {
    pub rotation_amount: Vec3,
    pub speed: f32,
    pub rotate_type: RotationType,
    pub label: ObjectLabel,
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
            .add_system(rotation_init)
            .add_system(perform_rotation)
            .add_system(translation_init)
            .add_system(perform_translation);
    }
}
fn translation_init(
    mut commands: Commands,
    mut animation_events: EventReader<AnimationEvent>,
    transforms: Query<(Entity, &Transform, &ObjectLabel)>,
) {
    for animation_event in &mut animation_events {
        if let AnimationEvent::Translate(movement_event) = animation_event {
            for (entity, transform, object_label) in &transforms {
                if object_label == &movement_event.label {
                    commands.entity(entity).insert(Translate {
                        waypoints: movement_event
                            .waypoints
                            .iter()
                            .map(|wp| match movement_event.movement_type {
                                TranslationType::LinearAbsolute => *wp,
                                TranslationType::LinearRelative => *wp + transform.translation,
                            })
                            .collect(),
                        speed: movement_event.speed,
                        movement_type: movement_event.movement_type,
                    });
                }
            }
        }
    }
}
fn rotation_init(
    mut commands: Commands,
    mut animation_events: EventReader<AnimationEvent>,
    transforms: Query<(Entity, &Transform, &ObjectLabel)>,
) {
    for animation_event in &mut animation_events {
        if let AnimationEvent::Rotate(rotation_event) = animation_event {
            for (entity, transform, object_label) in &transforms {
                if object_label == &rotation_event.label {
                    println!("Got {:?}", rotation_event.rotation_amount);
                    commands.entity(entity).insert(Rotate {
                        rotation_amount: match rotation_event.rotate_type {
                            RotationType::LinearAbsolute => rotation_event.rotation_amount,
                            RotationType::LinearRelative => {
                                let (euler_x, euler_y, euler_z): (f32, f32, f32) =
                                    transform.rotation.to_euler(EulerRot::XYZ);
                                let euler_rotation: Vec3 = Vec3 {
                                    x: euler_x,
                                    y: euler_y,
                                    z: euler_z,
                                };
                                rotation_event.rotation_amount + euler_rotation
                            }
                        },
                        speed: rotation_event.speed,
                        rotation_type: rotation_event.rotate_type,
                    });
                }
            }
        }
    }
}
fn perform_rotation(
    mut commands: Commands,
    time: Res<Time>,
    mut objects: Query<(Entity, &mut Transform, &mut Rotate)>,
) {
    for (entity, mut transform, movement) in objects.iter_mut() {
        match movement.rotation_type {
            RotationType::LinearAbsolute | RotationType::LinearRelative => {
                let target_rotation = movement.rotation_amount;
                let (euler_x, euler_y, euler_z): (f32, f32, f32) =
                    transform.rotation.to_euler(EulerRot::XYZ);
                let euler_rotation: Vec3 = Vec3 {
                    x: euler_x,
                    y: euler_y,
                    z: euler_z,
                };
                let mut rotation_amount: Vec3 = (target_rotation - euler_rotation).normalize()
                    * time.delta_seconds()
                    * movement.speed;
                let new_rotation: Vec3;
                if rotation_amount.is_nan()
                    || rotation_amount.length() > (target_rotation - euler_rotation).length()
                {
                    new_rotation = target_rotation;
                    commands.entity(entity).remove::<Rotate>();
                } else {
                    new_rotation = euler_rotation + rotation_amount;
                }

                *transform = transform.with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    new_rotation.x,
                    new_rotation.y,
                    new_rotation.z,
                ));
                println!("New Transform {:#?}", new_rotation);
                println!("Transform after  {:#?}", transform.rotation);
                println!("Rotation after {:?}\n", transform.rotation.xyz());
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
            TranslationType::LinearAbsolute | TranslationType::LinearRelative => {
                let target: Vec3 = movement.waypoints[0];
                let move_amount: Vec3 = (target - transform.translation).normalize()
                    * time.delta_seconds()
                    * movement.speed;

                let new_pos: Vec3;
                if move_amount.is_nan()
                    || move_amount.length() > (target - transform.translation).length()
                {
                    new_pos = target;
                    commands.entity(entity).remove::<Translate>();
                } else {
                    new_pos = transform.translation + move_amount;
                }
                transform.translation = new_pos;
            }
        }
    }
}
