use bevy::{app::PluginGroupBuilder, prelude::*, scene::SceneBundle};
use fade::InitialSettings;
pub mod animation;
pub mod change_background;
pub mod fade;
mod material_interaction;
pub mod movement;
pub struct ElaphosDefaultPlugins;

impl PluginGroup for ElaphosDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(fade::ElaphosFadePlugin)
            .add(movement::ElaphosMovementPlugin)
            .add(change_background::ElaphosBackgroundPlugin)
    }
}
/// A String Label for all animatable entities in a scene, by which they will be selected
#[derive(Component, Eq, PartialEq, Debug)]
pub struct ObjectLabel(pub String);
/// A wrapper for the Bevy SceneBundle for 3D scenes
#[derive(Bundle)]
pub struct ElaphosSceneBundle {
    /// The core SceneBundle
    pub scene_bundle: SceneBundle,
    /// The Label attached to the SceneBundle
    pub object_label: ObjectLabel,
    /// The settings to initialise the scene to
    pub initial_settings: InitialSettings,
}
impl ElaphosSceneBundle {
    /// Creates a Scenebundle with no initial settings
    pub fn from_scene_bundle(scene_bundle: SceneBundle, name: &str) -> ElaphosSceneBundle {
        Self {
            scene_bundle,
            object_label: ObjectLabel(name.to_owned()),
            initial_settings: InitialSettings(Vec::new()),
        }
    }
}
