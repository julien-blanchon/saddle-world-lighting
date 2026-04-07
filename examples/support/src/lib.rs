use bevy::prelude::*;
use saddle_world_time::TimeOfDay;
use saddle_world_lighting::{CelestialState, DayNightLighting};

#[derive(Component)]
pub struct LightingOverlay;

pub fn spawn_lighting_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Lighting Overlay"),
        LightingOverlay,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        Text::new("Lighting: --"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

pub fn update_lighting_overlay(
    time_of_day: Res<TimeOfDay>,
    celestial: Res<CelestialState>,
    lighting: Res<DayNightLighting>,
    mut query: Query<&mut Text, With<LightingOverlay>>,
) {
    for mut text in &mut query {
        let h = time_of_day.hour as u32;
        let m = ((time_of_day.hour - h as f32) * 60.0) as u32;
        text.0 = format!(
            "Time: {:02}:{:02}  Phase: {:?}\nSun elev: {:.1}\u{00b0}  Moon: {:?}\nSun lux: {:.0}  Ambient: {:.0}%\nFog vis: {:.0}m  EV100: {:.1}",
            h, m, celestial.phase,
            celestial.sun_elevation_degrees,
            celestial.moon_phase,
            lighting.sun_illuminance_lux,
            lighting.ambient_brightness * 100.0,
            lighting.fog_visibility,
            lighting.suggested_exposure_ev100,
        );
    }
}

pub fn spawn_outdoor_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Mesh::from(Plane3d::new(Vec3::Y, Vec2::splat(25.0))))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.55, 0.30),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // A few colored boxes
    let colors = [
        Color::srgb(0.8, 0.3, 0.3),
        Color::srgb(0.3, 0.7, 0.3),
        Color::srgb(0.3, 0.3, 0.8),
        Color::srgb(0.8, 0.7, 0.2),
        Color::srgb(0.6, 0.3, 0.7),
    ];
    for (i, color) in colors.iter().enumerate() {
        let x = (i as f32 - 2.0) * 3.5;
        commands.spawn((
            Name::new(format!("Box {}", i + 1)),
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(1.5, 1.5, 1.5)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: *color,
                ..default()
            })),
            Transform::from_xyz(x, 0.75, 0.0),
        ));
    }
}
