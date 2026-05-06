use bevy::prelude::*;
use crate::SimParams;

// UI elements
#[derive(Component)]
pub struct ParamDisplay;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextFont {
            font_size: 10.0,
            ..default()
        },
        Visibility::Hidden,
        ParamDisplay,
    ));
}

pub fn text_toggle(
    mut visibility: Query<&mut Visibility, With<ParamDisplay>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Tab) {
        if let Ok(mut visible) = visibility.single_mut() {
            *visible = match *visible {
                Visibility::Visible => Visibility::Hidden,
                _ => Visibility::Visible,
            }
        }
    }
}

pub fn text_update_system(
    mut query: Query<&mut Text, With<ParamDisplay>>,
    params: Res<SimParams>,
) {
    let perception_radius = params.perception_radius;
    let separation_radius = params.separation_radius;
    let align_weight = params.align_weight;
    let cohere_weight = params.cohere_weight;
    let avoid_weight = params.avoid_weight;

    if let Ok(mut text) = query.single_mut() {
        **text = format!("
        Perception: {perception_radius:.2}
        Separation: {separation_radius:.2}
        Alignment:  {align_weight:.2}
        Cohesion:   {cohere_weight:.2}
        Avoidance:  {avoid_weight:.2}
        ");
    }
}