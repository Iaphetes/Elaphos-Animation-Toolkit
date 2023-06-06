use bevy::{
    app::PluginGroupBuilder,
    ecs::system::Command,
    prelude::*,
    scene::{SceneBundle, SceneInstance},
};
use fade::{ElaphosSetting, InitialSettings};
pub mod animation;
pub mod change_background;
pub mod fade;
mod material_interaction;
pub mod movement;
// pub struct ElaphosInitPlugin;

// impl Plugin for ElaphosInitPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system(init);
//     }
// }
pub struct ElaphosDefaultPlugins;

impl PluginGroup for ElaphosDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(fade::ElaphosFadePlugin)
            .add(movement::ElaphosMovementPlugin)
            .add(change_background::ElaphosBackgroundPlugin)
    }
}
#[derive(Component, Eq, PartialEq, Debug)]
pub struct ObjectLabel(pub String);
#[derive(Bundle)]
pub struct ElaphosBundle3D {
    scene_bundle: SceneBundle,
    object_label: ObjectLabel,
    initial_settings: InitialSettings,
}
impl ElaphosBundle3D {
    pub fn from_scene_bundle(scene_bundle: SceneBundle, name: &str) -> ElaphosBundle3D {
        Self {
            scene_bundle,
            object_label: ObjectLabel(name.to_owned()),
            initial_settings: InitialSettings(vec![
                ElaphosSetting::AlphaMode(AlphaMode::Blend),
                ElaphosSetting::DoubleSided,
            ]),
        }
    }
}
