use crate::{
    change_background::BackgroundEvent,
    fade::FadeEvent,
    movement::{RotateEvent, TranslateEvent},
};

pub enum ElaphosAnimationEvent {
    Fade(FadeEvent),
    Translate(TranslateEvent),
    Rotate(RotateEvent),
    Background(BackgroundEvent),
}
