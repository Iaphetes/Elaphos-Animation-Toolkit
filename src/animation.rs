use bevy::prelude::*;

use crate::{
    change_background::{BackgroundEvent, BackgroundPlugin},
    fade::{FadeEvent, FadePlugin},
    movement::{MovementPlugin, RotateEvent, TranslateEvent},
};

#[derive(Component, Eq, PartialEq)]
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
        app.add_plugin(FadePlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(BackgroundPlugin);
    }
}
