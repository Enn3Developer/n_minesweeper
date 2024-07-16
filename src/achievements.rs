use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Resource)]
pub struct Achievements {}
