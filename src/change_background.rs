use bevy::prelude::*;

use crate::animation::{AnimationEvent, ObjectLabel};
pub struct BackgroundEvent {
    pub speed: f32,
    pub color: Color,
}
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvent>()
            .add_system(fade_background);
    }
}
fn fade_background(
    mut commands: Commands,
    mut animation_events: EventReader<AnimationEvent>,
    mut background_color: ResMut<ClearColor>,
    mut target_parameters: Local<Option<(Color, f32)>>,
    time: Res<Time>,
) {
    for animation_event in &mut animation_events {
        if let AnimationEvent::Background(background_event) = animation_event {
            *target_parameters = Some((background_event.color, background_event.speed))
        }
    }
    if let Some((color, speed)) = *target_parameters {
        let color_change: Vec4 = (Vec4::from(color) - Vec4::from(background_color.0)).normalize()
            * time.delta_seconds()
            * speed;
        if color_change.is_nan()
            || color_change.length() > (Vec4::from(color) - Vec4::from(background_color.0)).length()
        {
            **background_color = color;
            *target_parameters = None;
        } else {
            **background_color += color_change;
        }
    }
}
