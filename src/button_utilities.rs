use bevy::prelude::*;
use realsense_wrapper::{format::Rs2Format, stream::Rs2StreamKind};

use crate::realsense_bevy::RestartRealsenseEvent;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const FORMAT_ARRAY: [Rs2Format; 3] = [Rs2Format::RGB8, Rs2Format::Y16, Rs2Format::Z16];

//I think this can just be 1 number that is converted to the rs2_format. Then at the end of the list just circle back
//I might even need some format struct wrapper for this task
#[derive(Resource)]
pub struct FormatSelectionResource {
    pub index: usize,
    pub format: Rs2Format,
    pub stream: Rs2StreamKind,
}

impl FormatSelectionResource {
    //For now I am just going to make the default RGB8 and Color
    pub fn new(index: usize) -> Self {
        FormatSelectionResource {
            index: index,
            format: FORMAT_ARRAY[index],
            stream: match_stream(FORMAT_ARRAY[index]),
        }
    }
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
    mut current_format: ResMut<FormatSelectionResource>,
    mut restart_realsense: EventWriter<RestartRealsenseEvent>,
) {
    for (_flag, interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Format".to_string();
                *color = PRESSED_BUTTON.into();
                format_update(&mut current_format);
                restart_realsense.send(RestartRealsenseEvent { does_exist: true })
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

//Lazy implementation?? Should I make a setter for the resource?
fn format_update(current_format: &mut ResMut<FormatSelectionResource>) {
    if current_format.index == (FORMAT_ARRAY.len() - 1) {
        current_format.index = 0;
    } else {
        current_format.index += 1;
    }
    current_format.format = FORMAT_ARRAY[current_format.index];
    current_format.stream = match_stream(current_format.format);

    println!("format: {:?}", current_format.format);
    println!("stream: {:?}", current_format.stream);
}

fn match_stream(format: Rs2Format) -> Rs2StreamKind {
    match format {
        Rs2Format::RGB8 => return Rs2StreamKind::Color,
        Rs2Format::Y16 => return Rs2StreamKind::Color,
        Rs2Format::Z16 => return Rs2StreamKind::Depth,
        _ => Rs2StreamKind::Any,
    }
}
