use bevy::prelude::*;

#[derive(Component)]
struct MoneyText;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add money text
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Money: 0",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(3.0),
            right: Val::Px(5.0),
            ..default()
        }),
        MoneyText,
    ));
}
