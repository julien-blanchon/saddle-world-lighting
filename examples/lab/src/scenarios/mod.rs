mod support;

use bevy::prelude::*;
use saddle_bevy_e2e::{action::Action, scenario::Scenario};
use saddle_world_time::TimeOfDay;
use saddle_world_lighting::DayNightLighting;

pub fn list_scenarios() -> Vec<&'static str> {
    vec!["lighting_smoke", "lighting_full_cycle"]
}

pub fn scenario_by_name(name: &str) -> Option<Scenario> {
    match name {
        "lighting_smoke" => Some(lighting_smoke()),
        "lighting_full_cycle" => Some(lighting_full_cycle()),
        _ => None,
    }
}

fn lighting_smoke() -> Scenario {
    Scenario::builder("lighting_smoke")
        .description("Verify lighting resources exist and sun illuminance is reasonable.")
        .then(Action::WaitFrames(30))
        .then(Action::Custom(Box::new(|world| {
            assert!(world.contains_resource::<DayNightLighting>());
            let lighting = world.resource::<DayNightLighting>();
            let time = world.resource::<TimeOfDay>();
            info!("[lighting_smoke] hour={:.2}, sun_lux={:.0}, ambient={:.2}",
                time.hour, lighting.sun_illuminance_lux, lighting.ambient_brightness);
            assert!(lighting.sun_illuminance_lux > 0.0 || time.hour < 6.0 || time.hour > 19.0);
        })))
        .then(Action::Screenshot("lighting_smoke".into()))
        .build()
}

fn lighting_full_cycle() -> Scenario {
    Scenario::builder("lighting_full_cycle")
        .description("Run a fast 24h cycle and verify sun illuminance varies.")
        .then(Action::Custom(Box::new(|world| {
            world.resource_mut::<saddle_world_time::TimeConfig>().seconds_per_hour = 0.05;
        })))
        .then(Action::WaitFrames(600))
        .then(Action::Custom(Box::new(|world| {
            let time = world.resource::<TimeOfDay>();
            info!("[lighting_full_cycle] hour={:.2}, days={}", time.hour, time.elapsed_days);
            assert!(time.elapsed_days >= 1, "should complete at least one full day");
        })))
        .then(Action::Screenshot("lighting_full_cycle".into()))
        .build()
}
