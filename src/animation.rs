use bevy::prelude::Component;

use crate::{
    change_background::BackgroundEvent,
    fade::TextFadeEvent,
    movement::{RotateEvent, TranslateEvent},
};

#[derive(Component, Eq, PartialEq)]
pub struct ObjectLabel(pub String);
pub enum AnimationEvent {
    TextFade(TextFadeEvent),
    Translate(TranslateEvent),
    Rotate(RotateEvent),
    Background(BackgroundEvent),
}
