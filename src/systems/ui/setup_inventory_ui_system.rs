use bevy::prelude::*;

use crate::components::InventoryCell;

pub fn setup_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a grid
    build_inventory_ui(&mut commands, asset_server);
}

pub fn build_inventory_ui(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity {
    // Load the item texture and store it with a handle
    //let item_texture_handle = asset_server.load("item_texture.png");

    let inventory_ui_entity = commands
        .spawn(NodeBundle {
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
        })
        .with_children(|builder| {
            let rows = 4;
            let columns = 10;

            //let square_size = 100.0 / f32::max(rows as f32, columns as f32) as f32;
            let ratio = columns as f32 / rows as f32;
            // Grid
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        aspect_ratio: Some(ratio),
                        display: Display::Grid,
                        padding: UiRect::all(Val::Px(5.0)),
                        // 4 Columns
                        grid_template_columns: RepeatedGridTrack::flex(columns, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(rows, 1.0),
                        row_gap: Val::Px(2.0),
                        column_gap: Val::Px(2.0),
                        ..default()
                    },
                    // background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                })
                .with_children(|builder| {
                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                    for _ in 0..rows * columns {
                        item_rect(builder, Color::ORANGE, &font);
                    }
                });
        })
        .id();

    inventory_ui_entity
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
                background_color: BackgroundColor(Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.5 }),
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
