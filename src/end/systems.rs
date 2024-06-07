use crate::{EndState, NStopWatch};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub fn show_text(
    mut commands: Commands,
    end_state: Res<State<EndState>>,
    stop_watch: Res<NStopWatch>,
) {
    let text = match end_state.get() {
        EndState::NotEnded => unreachable!(),
        EndState::Win => "You won!",
        EndState::Lose => "You lose!",
    };

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font_size: 32.0,
                ..default()
            },
        ),
        transform: Transform::from_xyz(300.0, 325.0, 0.0),
        text_anchor: Anchor::Center,
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("{:0.2} seconds", stop_watch.0.elapsed_secs()),
            TextStyle {
                font_size: 32.0,
                ..default()
            },
        ),
        transform: Transform::from_xyz(300.0, 275.0, 0.0),
        text_anchor: Anchor::Center,
        ..default()
    });
}

pub fn cleanup(texts: Query<Entity, With<Text>>, mut commands: Commands) {
    texts
        .iter()
        .for_each(|text| commands.entity(text).despawn());
}
