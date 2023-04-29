use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use button_utilities::{button_system, button_system_cycle_format, FormatButton, NORMAL_BUTTON};
use realsense_bevy::{
    update_display_system, update_frame_buffer, FrameBufferResource, RealsensePlugin,
    RealsenseResource,
};

mod button_utilities;
mod realsense_bevy;

#[derive(Component)]
pub struct FeedImage(pub bool);

fn main() {
    App::new()
        .insert_resource(FrameBufferResource::new())
        .insert_resource(RealsenseResource::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(realsense_bevy::realsense_start_system)
        .add_system(button_system)
        .add_system(button_system_cycle_format)
        .add_system(update_frame_buffer)
        .add_system(update_display_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //ui camera
    commands.spawn(Camera2dBundle::default());

    //This is the format button
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                //center button
                margin: UiRect {
                    left: Val::Percent(75.0),
                    ..default()
                },
                //horizontally center child text
                justify_content: JustifyContent::Center,
                //vertically center child text
                align_items: AlignItems::Center,
                //Puts the button in the bottom right
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(30.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(FormatButton(true))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0., 0.0),
                },
            ));
        });

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                //center button
                margin: UiRect {
                    left: Val::Percent(75.0),
                    ..default()
                },
                //horizontally center child text
                justify_content: JustifyContent::Center,
                //vertically center child text
                align_items: AlignItems::Center,
                //Puts the button in the bottom right
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(50.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.0, 0.9, 0.0),
                },
            ));
        });

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                //center button
                margin: UiRect {
                    left: Val::Percent(75.0),
                    ..default()
                },
                //horizontally center child text
                justify_content: JustifyContent::Center,
                //vertically center child text
                align_items: AlignItems::Center,
                //Puts the button in the bottom right
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(70.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.0, 0., 0.9),
                },
            ));
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(640.0), Val::Px(480.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(25.0),
                    left: Val::Percent(25.0),
                    ..default()
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(FeedImage(true))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    ..default()
                },
                image: asset_server.load("decent_images/pride_flag.png").into(),
                ..default()
            });
        });
}
