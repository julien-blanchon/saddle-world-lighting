use bevy::prelude::*;

use saddle_world_time::TimePlugin;

use crate::{LightingConfig, LightingPlugin, ManagedLightConfig, Moon, Sun};

#[test]
fn plugin_builds_and_initializes_resources() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TimePlugin::default(),
        LightingPlugin::default(),
    ));
    app.update();

    assert!(
        app.world()
            .contains_resource::<saddle_world_time::TimeOfDay>()
    );
    assert!(app.world().contains_resource::<crate::CelestialState>());
    assert!(
        app.world()
            .contains_resource::<crate::DayNightLighting>()
    );
    assert!(
        app.world()
            .contains_resource::<crate::DayNightDiagnostics>()
    );
}

#[test]
fn plugin_supports_existing_managed_lights() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TimePlugin::default(),
        LightingPlugin::default(),
    ));
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((
            Name::new("Test Sun"),
            Sun,
            DirectionalLight::default(),
            Transform::default(),
        ));
        commands.spawn((
            Name::new("Test Moon"),
            Moon,
            DirectionalLight::default(),
            Transform::default(),
        ));
    });

    app.update();

    let mut query = app.world_mut().query::<(&Sun, &DirectionalLight)>();
    let (_, light) = query
        .single(app.world())
        .expect("a managed sun light should exist");
    assert!(light.illuminance >= 0.0);
}

#[test]
fn plugin_does_not_require_auto_spawned_lights() {
    let mut app = App::new();
    let config = LightingConfig {
        managed_lights: ManagedLightConfig { auto_spawn: false },
        ..default()
    };
    app.add_plugins((
        MinimalPlugins,
        TimePlugin::default(),
        LightingPlugin::default().with_config(config),
    ));
    app.update();
    app.update();

    let sun_count = {
        let mut query = app.world_mut().query::<&Sun>();
        query.iter(app.world()).count()
    };
    let moon_count = {
        let mut query = app.world_mut().query::<&Moon>();
        query.iter(app.world()).count()
    };
    assert_eq!(sun_count, 0);
    assert_eq!(moon_count, 0);
}
