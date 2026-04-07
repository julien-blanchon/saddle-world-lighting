use bevy::prelude::*;
use saddle_world_time::{TimeActive, TimeConfig, TimePlugin, TimeReactive};
use saddle_world_lighting::{LightingCamera, LightingConfig, LightingPlugin, Sun, Moon};
use saddle_world_lighting_example_support as support;

fn main() {
    let time_config = TimeConfig {
        initial_time: 17.0,
        seconds_per_hour: 1.5,
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.56, 0.63, 0.72)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lighting \u{2014} Street Lights".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TimePlugin::default().with_config(time_config))
        .add_plugins(LightingPlugin::default().with_config(LightingConfig::default()))
        .add_systems(Startup, (setup, support::spawn_lighting_overlay, support::spawn_outdoor_scene))
        .add_systems(Update, (support::update_lighting_overlay, update_street_lights))
        .run();
}

#[derive(Component)]
struct StreetLamp;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    // Spawn street lamps with TimeReactive (night-active)
    let lamp_positions = [
        Vec3::new(-8.0, 0.0, -4.0),
        Vec3::new(-4.0, 0.0, -4.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(4.0, 0.0, -4.0),
        Vec3::new(8.0, 0.0, -4.0),
    ];

    let pole_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.3),
        ..default()
    });

    for (i, pos) in lamp_positions.iter().enumerate() {
        // Lamp pole
        commands.spawn((
            Name::new(format!("Lamp Pole {}", i + 1)),
            Mesh3d(meshes.add(Mesh::from(Cylinder::new(0.08, 3.5)))),
            MeshMaterial3d(pole_material.clone()),
            Transform::from_translation(*pos + Vec3::Y * 1.75),
        ));

        // Lamp head with point light (night-active via TimeReactive)
        commands
            .spawn((
                Name::new(format!("Street Lamp {}", i + 1)),
                StreetLamp,
                TimeReactive::night_active(),
                PointLight {
                    color: Color::srgb(1.0, 0.85, 0.55),
                    intensity: 0.0,
                    range: 12.0,
                    shadows_enabled: false,
                    ..default()
                },
                Transform::from_translation(*pos + Vec3::Y * 3.6),
            ));
    }
}

fn update_street_lights(
    mut query: Query<(&mut PointLight, Has<TimeActive>), With<StreetLamp>>,
) {
    for (mut light, is_active) in &mut query {
        let target_intensity = if is_active { 8000.0 } else { 0.0 };
        // Smooth fade
        light.intensity += (target_intensity - light.intensity) * 0.08;
    }
}
