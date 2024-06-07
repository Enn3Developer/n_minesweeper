use crate::EndState;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub fn show_text(mut commands: Commands, end_state: Res<State<EndState>>) {
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
        transform: Transform::from_xyz(300.0, 300.0, 0.0),
        text_anchor: Anchor::Center,
        ..default()
    });
}
