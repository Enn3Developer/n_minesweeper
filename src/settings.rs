use bevy::prelude::{AppExit, Commands, EventReader, Res, ResMut, Resource};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Resource)]
pub struct Settings {
    pub name: String,
}

pub fn setup(mut pkv: ResMut<PkvStore>, mut commands: Commands) {
    if let Ok(settings) = pkv.get::<Settings>("settings") {
        commands.insert_resource(settings);
    } else {
        let mut settings = Settings::default();
        pkv.set("settings", &mut settings)
            .expect("can't set default settings");
        commands.insert_resource(settings);
    }
}

pub fn save(mut pkv: ResMut<PkvStore>, settings: Res<Settings>, app_exit: EventReader<AppExit>) {
    if !app_exit.is_empty() {
        println!("saving");
        pkv.set("settings", settings.into_inner())
            .expect("can't save settings");
    }
}
