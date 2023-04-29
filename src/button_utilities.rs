use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

//I think this can just be 1 number that is converted to the rs2_format. Then at the end of the list just circle back
//I might even need some format struct wrapper for this task
#[derive(Resource)]
pub struct FormatSelectionResource {
    pub index: i32,
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

//This will need a button click event
pub fn format_update(mut current_format: ResMut<FormatSelectionResource>) {
    if current_format.index == 5
    //this can be any value really just some limit value to cycle correctly
    {
        current_format.index = 0;
    } else {
        current_format.index += 1;
    }
}
