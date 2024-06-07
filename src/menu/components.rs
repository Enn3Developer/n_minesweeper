use bevy::prelude::*;

#[derive(Component)]
pub enum ButtonAction {
    Play,
    Exit,
}

#[derive(Component)]
pub struct SelectedOption;
