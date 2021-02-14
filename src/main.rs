use bevy::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod overworld;
use overworld::*;

fn main() {
    App::build()
        .insert_resource(ReportExecutionOrderAmbiguities)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: "SOTORA™ PRE-ALPHA".into(),
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_overworld.system())
        .add_system(move_player.system())
        .add_system(rotate_player.system())
        .run();
}
