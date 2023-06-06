use bevy::{prelude::*, scene::SceneInstance};

use crate::ObjectLabel;

pub fn get_attached_standardmaterial(
    entity: Entity,
    material_handles: &Query<(&Name, &mut Handle<StandardMaterial>)>,
    material_assets: &ResMut<Assets<StandardMaterial>>,
    scene_instances: &Query<&SceneInstance>,
    scene_spawner: &Res<SceneSpawner>,
) -> Result<Entity, String> {
    match scene_instances.get(entity) {
        Ok(scene_instance) => {
            for scene_entity in scene_spawner.iter_instance_entities(**scene_instance.to_owned()) {
                // if let Ok((name, mut material_handle)) = material_handles.get(scene_entity) {
                //     return Ok((name.to_string(), material_handle.clone()));
                // }
                if material_handles.contains(scene_entity) {
                    return Ok(scene_entity);
                }
            }
        }
        Err(error) => {
            return Err(format!("No scene attached to entity {:?}", error));
        }
    }
    return Err("No matching standard material found".to_owned());
}
