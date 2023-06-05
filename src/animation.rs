use bevy::prelude::*;

use crate::{
    change_background::{BackgroundEvent, ElaphosBackgroundPlugin},
    fade::{ElaphosFadePlugin, FadeEvent},
    movement::{ElaphosMovementPlugin, RotateEvent, TranslateEvent},
};

#[derive(Component, Eq, PartialEq, Debug)]
pub struct ObjectLabel(pub String);
pub enum ElaphosAnimationEvent {
    Fade(FadeEvent),
    Translate(TranslateEvent),
    Rotate(RotateEvent),
    Background(BackgroundEvent),
}
pub struct ElaphosDefaultPlugins;

impl Plugin for ElaphosDefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugin(ElaphosFadePlugin)
            .add_plugin(ElaphosMovementPlugin)
            .add_plugin(ElaphosBackgroundPlugin);
    }
}
