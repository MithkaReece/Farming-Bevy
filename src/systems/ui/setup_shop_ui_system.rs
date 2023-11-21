use bevy::prelude::*;

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

use std::fs;

pub fn setup_shop_ui(mut commands: Commands, asset_server: Res<AssetServer>) {

    /*let json_str = fs::read_to_string("assets/tiles_config.json");
    match json_str {
        Ok(data) => {
            let res: TilesData = serde_json::from_str(&data).unwrap();
            tiles_data.tiles = res.tiles;
        }
        Err(e) => {
            println!("Couldn't find tiles_config.json");
        }
    }*/

    /*let font = asset_server.load("fonts/FiraSans-Bold.ttf");
     
    commands.spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::Rgba {
                red: 0.0,
                green: 255.0,
                blue: 0.0,
                alpha: 255.0,
            }
            .into(),
            ..default()
        },
    )).with_children(|builder| {
        // Title
        builder.spawn((
            TextBundle::from_section(
                "Shop list",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 25.,
                    ..default()
                },
            ),
            Label,
        ));

        // List with hidden overflow
        builder.spawn(NodeBundle {
             style: Style {
                 flex_direction: FlexDirection::Column,
                 align_self: AlignSelf::Stretch,
                 height: Val::Percent(50.),
                 overflow: Overflow::clip_y(),
                 ..default()
             },
             background_color: Color::rgb(0.10, 0.10, 0.10).into(),
             ..default()
         })
         .with_children(|builder| {
             // Moving panel
             builder.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(1.0, 0.10, 0.10).into(),
                    ..default()
                },
                //ScrollingList::default(),
                AccessibilityNode(NodeBuilder::new(Role::List)),
                ))
                .with_children(|builder| {
                    shop_row_titles(builder, &font);
                    // List items
                    for i in 0..30 {
                        shop_row(builder, &font, format!("Type {i}").as_str(), format!("Price {i}").as_str());
                    }
                });
         });
    });*/
}

fn shop_row_titles(builder: &mut ChildBuilder, font: &Handle<Font>){
    builder.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            background_color: Color::rgb(0.0, 0.0, 1.0).into(),
            ..default()
        },
    )).with_children(|builder| {
        builder.spawn((
            TextBundle::from_section(
                "Type",
                TextStyle {
                    font: font.clone(),
                    font_size: 25.,
                    ..default()
                },
            ),
            Label,
        ));
        builder.spawn((
            TextBundle::from_section(
                "Price",
                TextStyle {
                    font: font.clone(),
                    font_size: 25.,
                    ..default()
                },
            ),
            Label,
        ));
        builder.spawn((
            TextBundle::from_section(
                "Buy buttons",
                TextStyle {
                    font: font.clone(),
                    font_size: 25.,
                    ..default()
                },
            ),
            Label,
        ));
    });
}

fn shop_row(builder: &mut ChildBuilder, font: &Handle<Font>, item_type: &str, item_price: &str) {
    builder.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            background_color: Color::rgb(0.0, 0.0, 1.0).into(),
            ..default()
        },
    )).with_children(|builder| {
        builder.spawn((
            TextBundle::from_section(
                item_type,
                TextStyle {
                    font: font.clone(),
                    font_size: 20.,
                    ..default()
                },
            ),
            Label,
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        ));
        builder.spawn((
            TextBundle::from_section(
                item_price,
                TextStyle {
                    font: font.clone(),
                    font_size: 20.,
                    ..default()
                },
            ),
            Label,
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        ));

        builder.spawn(ButtonBundle {
            ..default()
        }).with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    format!("BUY"),
                    TextStyle {
                        font:font.clone(),
                        font_size: 20.,
                        ..default()
                    },
                ),
                Label,
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            ));
        });

        
    });
}