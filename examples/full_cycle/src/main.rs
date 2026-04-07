use bevy::prelude::*;
use saddle_world_time::{TimeConfig, TimePlugin};
use saddle_world_lighting::{LightingCamera, LightingConfig, LightingPlugin, Sun, Moon};
use saddle_world_lighting_example_support as support;

fn main() {
    let time_config = TimeConfig {
        initial_time: 5.5,
        seconds_per_hour: 0.75,
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.56, 0.63, 0.72)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lighting \u{2014} Full Cycle".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TimePlugin::default().with_config(time_config))
        .add_plugins(LightingPlugin::default().with_config(LightingConfig::default()))
        .add_systems(Startup, (setup, support::spawn_lighting_overlay, support::spawn_outdoor_scene))
        .add_systems(Update, support::update_lighting_overlay)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Camera"),
        Camera3d::default(),
        LightingCamera::default(),
        Transform::from_xyz(0.0, 5.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Sun"),
        Sun,
        DirectionalLight {
            illuminance: 0.0,
            shadows_enabled: true,
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Moon"),
        Moon,
        DirectionalLight {
            illuminance: 0.0,
            shadows_enabled: false,
            ..default()
        },
    ));
}
