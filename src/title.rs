use bevy::{app::AppExit, ecs::event::Event, prelude::*, utils::tracing::event};

const NORMAL_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
const HOVERED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);

pub fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(100.0),
                },
                padding: UiRect {
                    left: Val::Percent(2.0),
                    right: Val::Percent(2.0),
                    top: Val::Percent(1.0),
                    bottom: Val::Percent(1.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "New Game",
                TextStyle {
                    font: asset_server.load("fonts\\NotoSerif-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(100.0),
                },
                padding: UiRect {
                    left: Val::Percent(2.0),
                    right: Val::Percent(2.0),
                    top: Val::Percent(1.0),
                    bottom: Val::Percent(1.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Load Game",
                TextStyle {
                    font: asset_server.load("fonts\\NotoSerif-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(100.0),
                },
                padding: UiRect {
                    left: Val::Percent(2.0),
                    right: Val::Percent(2.0),
                    top: Val::Percent(1.0),
                    bottom: Val::Percent(1.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "How to Play",
                TextStyle {
                    font: asset_server.load("fonts\\NotoSerif-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(100.0),
                },
                padding: UiRect {
                    left: Val::Percent(2.0),
                    right: Val::Percent(2.0),
                    top: Val::Percent(1.0),
                    bottom: Val::Percent(1.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Exit",
                TextStyle {
                    font: asset_server.load("fonts\\NotoSerif-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    // commands.spawn_bundle(TextBundle::from_section(
    //     "First time playing? Try the How to Play button!",
    //     style: Style {
    //         margin: UiRect {
    //             left: Val::Px(50.0),
    //             right: Val::Auto,
    //             top: Val::Px(50.0),
    //             bottom: Val::Auto,
    //         },
    //         ..default()
    //     };
    //     style: TextStyle {
    //         font: asset_server.load("fonts\\NotoSerif-Bold.ttf"),
    //         font_size: 40.0,
    //         ..default()
    //     };
    // ));
}

pub fn button_system_title(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut writer: EventWriter<AppExit>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                if text.sections[0].value == "Exit" {
                    writer.send(AppExit);
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
