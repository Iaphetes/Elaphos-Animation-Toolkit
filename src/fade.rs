use std::collections::HashMap;

use bevy::{
    prelude::*,
    scene::{scene_spawner, SceneInstance},
    text::Text,
};

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

#[derive(Component)]
struct OriginalAlphas(HashMap<String, f32>);

fn fade_init(
    mut commands: Commands,
    mut animation_events: EventReader<ElaphosAnimationEvent>,
    texts: Query<(Entity, &mut Text, &ObjectLabel)>,
    sprites: Query<(Entity, &mut Sprite, &ObjectLabel)>,
    models: Query<(Entity, &ObjectLabel), With<Handle<Scene>>>,
    mut material_handles: Query<(&Name, &mut Handle<StandardMaterial>)>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    original_alphas: Query<&OriginalAlphas>,
    scene_instances: Query<&SceneInstance>,
    scene_spawner: Res<SceneSpawner>,
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

            for (entity, object_label) in &models {
                let mut previous_alphas: HashMap<String, f32> = HashMap::new();
                match scene_instances.get(entity) {
                    Ok(scene_instance) => {
                        println!("Found scene instance");
                        for scene_entity in
                            scene_spawner.iter_instance_entities(**scene_instance.to_owned())
                        {
                            match material_handles.get_mut(scene_entity) {
                                Ok((name, mut material_handle)) => {
                                    println!("Name {name}");
                                    let mut new_material: Option<StandardMaterial> = None;
                                    if let Some(mut material) =
                                        material_assets.get_mut(&material_handle)
                                    {
                                        new_material = Some(material.clone());

                                        // material.base_color = material.base_color.with_a(0.0);
                                        // material.emissive = material.emissive.with_a(0.0);
                                        println!("Material {:#?}", material);
                                    }

                                    if let Some(mut new_material) = new_material {
                                        new_material.alpha_mode = AlphaMode::Blend;
                                        previous_alphas
                                            .insert(name.to_string(), new_material.base_color.a());
                                        if !original_alphas.contains(entity) {
                                            println!("Adding new material");
                                            *material_handle = material_assets.add(new_material);
                                        }
                                    }
                                }
                                Err(error) => {
                                    println!("No material attached to entity {:?}", error)
                                }
                            }
                        }
                    }
                    Err(error) => println!("No scene attached to entity {:?}", error),
                }

                println!("{:#?}", object_label);
                if !original_alphas.contains(entity) {
                    commands
                        .entity(entity)
                        .insert(OriginalAlphas(previous_alphas));
                }
                target_entity = Some(entity);
                target_color = None;
            }
            if let Some(entity) = target_entity {
                println!("Color change initiated");
                let mut fade_amount: f32;
                if let Some(color) = target_color {
                    fade_amount = color.a();
                    if fade_amount <= 0.0 {
                        fade_amount = 1.0;
                    }
                } else {
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
fn fade_alpha(color: &mut Color, fade: &Fade, delta_seconds: f32, target_alpha: f32) -> bool {
    *color = color.with_a(color.a() - fade.fade_amount * delta_seconds * fade.speed);
    if fade.speed > 0.0 && color.a() <= target_alpha {
        *color = color.with_a(target_alpha);
        true
    } else if fade.speed < 0.0 && color.a() >= target_alpha {
        *color = color.with_a(target_alpha);
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
    mut models: Query<(Entity, &Fade, &OriginalAlphas), With<Handle<Scene>>>,
    scene_instances: Query<&SceneInstance>,
    scene_spawner: Res<SceneSpawner>,
    // original_alphas: Query<&OriginalAlphas>,
    mut material_handles: Query<(&Name, &mut Handle<StandardMaterial>)>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut text, fade) in texts.iter_mut() {
        let mut all_sections_finished: bool = true;
        for section in text.sections.iter_mut() {
            if !fade_alpha(&mut section.style.color, fade, time.delta_seconds(), {
                if fade.speed > 0.0 {
                    0.0
                } else {
                    1.0
                }
            }) {
                all_sections_finished = false;
            }
        }
        if all_sections_finished {
            commands.entity(entity).remove::<Fade>();
        }
    }
    for (entity, mut sprite, fade) in sprites.iter_mut() {
        if fade_alpha(&mut sprite.color, fade, time.delta_seconds(), {
            if fade.speed > 0.0 {
                0.0
            } else {
                1.0
            }
        }) {
            commands.entity(entity).remove::<Fade>();
        }
    }
    for (entity, fade, original_alphas) in &mut models {
        let mut all_sections_finished: bool = true;
        match scene_instances.get(entity) {
            Ok(scene_instance) => {
                for scene_entity in
                    scene_spawner.iter_instance_entities(**scene_instance.to_owned())
                {
                    match material_handles.get_mut(scene_entity) {
                        Ok((name, mut material_handle)) => {
                            match material_assets.get_mut(&material_handle) {
                                Some(material) => {
                                    if fade.speed > 0.0 {
                                        // println!("Reducing");
                                        // println!("Alpha before: {}", material.base_color.a());
                                        // println!("Alpha after: {}", material.base_color.a());
                                        if !fade_alpha(
                                            &mut material.base_color,
                                            fade,
                                            time.delta_seconds(),
                                            0.0,
                                        ) {
                                            all_sections_finished = false;
                                        }
                                    } else {
                                        if let Some(alpha) =
                                            original_alphas.0.get(&name.to_string())
                                        {
                                            if !fade_alpha(
                                                &mut material.base_color,
                                                fade,
                                                time.delta_seconds(),
                                                *alpha,
                                            ) {
                                                all_sections_finished = false;
                                            }
                                        }
                                    }
                                }
                                None => {
                                    // println!("Invalid shader handle")
                                }
                            }
                        }
                        Err(error) => {
                            // println!("No material attached to entity {:?}", error)
                        }
                    }
                }
            }
            Err(error) => println!("No scene attached to entity {:?}", error),
        }

        if all_sections_finished {
            commands.entity(entity).remove::<Fade>();
        }
    }
}
