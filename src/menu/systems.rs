use crate::menu::components::*;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn init(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.resolution.set(600.0, 600.0);
    window.resize_constraints.min_height = 600.0;
    window.resize_constraints.min_width = 600.0;
    window.title = String::from("N Mines");
}

pub fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(600.0 / 2.0, 600.0 / 2.0, 0.0);
    camera.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    commands.spawn(camera);

    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let text_color = Color::rgb(0.9, 0.9, 0.9);

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: text_color,
        ..default()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "N Mines",
                            TextStyle {
                                font_size: 40.0,
                                color: text_color,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    for (action, text) in
                        [(ButtonAction::Play, "Play"), (ButtonAction::Exit, "Exit")]
                    {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(text, button_text_style.clone()),
                                    ..default()
                                });
                            });
                    }
                });
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

pub fn button_click(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                ButtonAction::Play => {
                    app_state.set(AppState::Playing);
                }
                ButtonAction::Exit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}

pub fn cleanup(nodes: Query<Entity, With<Node>>, mut commands: Commands) {
    nodes
        .iter()
        .for_each(|node| commands.entity(node).despawn());
}
