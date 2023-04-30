use bevy::prelude::*;
use realsense_wrapper::{format::Rs2Format, stream::Rs2StreamKind};

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const FORMAT_ARRAY: [Rs2Format; 3] = [Rs2Format::RGB8, Rs2Format::Y16, Rs2Format::Z16];

//I think this can just be 1 number that is converted to the rs2_format. Then at the end of the list just circle back
//I might even need some format struct wrapper for this task
#[derive(Resource)]
pub struct FormatSelectionResource {
    pub index: i32,
    pub stream: Rs2StreamKind,
}

#[derive(Component)]
pub struct FormatButton(pub bool);

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn button_system_cycle_format(
    mut interaction_query: Query<
        (&FormatButton, &Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (_flag, interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Format".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Format".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Format".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

//This will need a format button click event
pub fn format_update(mut current_format: ResMut<FormatSelectionResource>) {
    if current_format.index == FORMAT_ARRAY.len() as i32 {
        current_format.index = 0;
    } else {
        current_format.index += 1;
    }

    current_format.stream = match_stream(current_format.index);
}

fn match_stream(index: i32) -> Rs2StreamKind {
    match FORMAT_ARRAY[index as usize] {
        Rs2Format::RGB8 => return Rs2StreamKind::Color,
        Rs2Format::Y16 => return Rs2StreamKind::Color,
        Rs2Format::Any => return Rs2StreamKind::Color,
        _ => Rs2StreamKind::Any,
    }
}
