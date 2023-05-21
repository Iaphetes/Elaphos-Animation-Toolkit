use bevy::prelude::*;

use crate::animation::ElaphosAnimationEvent;
pub struct BackgroundEvent {
    pub speed: f32,
    pub color: Color,
}
pub struct ElaphosBackgroundPlugin;

impl Plugin for ElaphosBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ElaphosAnimationEvent>()
            .add_system(fade_background);
    }
}
fn fade_background(
    mut animation_events: EventReader<ElaphosAnimationEvent>,
    mut background_color: ResMut<ClearColor>,
    mut target_parameters: Local<Option<(Color, f32)>>,
    time: Res<Time>,
) {
    for animation_event in &mut animation_events {
        if let ElaphosAnimationEvent::Background(background_event) = animation_event {
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
