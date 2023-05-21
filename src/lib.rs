use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    text::Text,
};
mod animation;
mod fade;
mod movement;

mod change_background;
use animation::{AnimationEvent, ObjectLabel};
use change_background::{BackgroundEvent, BackgroundPlugin};
use fade::{FadeEvent, FadePlugin};
use movement::{MovementPlugin, RotateEvent, RotationType, TranslateEvent, TranslationType};
