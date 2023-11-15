use bevy::prelude::*;

pub fn setup_shop_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::Rgba {
                    red: 255.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0,
                }
                .into(),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn();
        });
}

fn item_rect(builder: &mut ChildBuilder, color: Color, font: &Handle<Font>) {
    builder
        .spawn((
            NodeBundle {
                style: Style {
                    // display: Display::Grid,
                    // padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 0.5,
                }),
                ..default()
            },
            InventoryCell,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "Name, 0",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style { ..default() }),
                InventoryCell,
            ));
        });
}

//System
fn ui_inventory(mut query: Query<&mut Text, With<InventoryCell>>) {
    for (mut text) in &mut query {
        println!("Test");
        text.sections[0].value = "Test".to_string();
        // Turn the text purple
        text.sections[0].style.color = Color::Rgba {
            red: 1.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        };
    }
}
