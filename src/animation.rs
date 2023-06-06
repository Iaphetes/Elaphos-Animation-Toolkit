use bevy::prelude::*;

use crate::{
    change_background::{BackgroundEvent, ElaphosBackgroundPlugin},
    fade::{ElaphosFadePlugin, FadeEvent},
    movement::{ElaphosMovementPlugin, RotateEvent, TranslateEvent},
    ObjectLabel,
};

pub enum ElaphosAnimationEvent {
    Fade(FadeEvent),
    Translate(TranslateEvent),
    Rotate(RotateEvent),
    Background(BackgroundEvent),
}
