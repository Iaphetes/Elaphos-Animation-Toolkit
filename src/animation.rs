use bevy::prelude::Component;

use crate::{fade::TextFadeEvent, movement::TranslateEvent};

#[derive(Component, Eq, PartialEq)]
pub struct ObjectLabel(pub String);
pub enum AnimationEvent {
    TextFade(TextFadeEvent),
    Movement(TranslateEvent),
}
