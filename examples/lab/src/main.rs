#[cfg(feature = "e2e")]
mod e2e;
#[cfg(feature = "e2e")]
mod scenarios;

use bevy::prelude::*;
use saddle_world_time::{TimeConfig, TimePlugin};
use saddle_world_lighting::{LightingCamera, LightingConfig, LightingPlugin, Sun, Moon};
use saddle_world_lighting_example_support as support;

fn main() {
    let time_config = TimeConfig {
        initial_time: 5.5,
        seconds_per_hour: 1.0,
        ..default()
    };

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.56, 0.63, 0.72)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Lighting Lab".into(),
            resolution: (1280, 720).into(),
            ..default()
        }),
        ..default()
    }));
    #[cfg(all(feature = "dev", not(target_arch = "wasm32")))]
    app.add_plugins(bevy_brp_extras::BrpExtrasPlugin::default());
    #[cfg(feature = "e2e")]
    app.add_plugins(e2e::E2EPlugin);
    app.add_plugins(TimePlugin::default().with_config(time_config));
    app.add_plugins(LightingPlugin::default().with_config(LightingConfig::default()));
    app.add_systems(Startup, (setup, support::spawn_lighting_overlay, support::spawn_outdoor_scene));
    app.add_systems(Update, support::update_lighting_overlay);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Lab Camera"),
        Camera3d::default(),
        LightingCamera::default(),
        Transform::from_xyz(0.0, 5.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((Name::new("Lab Sun"), Sun, DirectionalLight { illuminance: 0.0, shadows_enabled: true, ..default() }));
    commands.spawn((Name::new("Lab Moon"), Moon, DirectionalLight { illuminance: 0.0, shadows_enabled: false, ..default() }));
}
