mod celestial;
mod components;
mod config;
mod gradient;
mod lighting;
mod systems;

pub use celestial::{
    CelestialModel, CelestialSettings, CelestialState, MoonPhase, SeasonSettings,
    solar_daylight_window, solve_celestial_state,
};
pub use components::{LightingCamera, Moon, Sun};
pub use config::{
    AtmosphereTuning, LightingConfig, ManagedLightConfig, ShadowConfig, SmoothingConfig,
    WriteThresholds,
};
pub use gradient::{ColorGradient, ColorKeyframe, ScalarGradient, ScalarKeyframe};
pub use lighting::{
    DayNightDiagnostics, DayNightLighting, LightingProfile, WeatherModulation, kelvin_to_color,
    resolve_lighting,
};

// Re-export key types from saddle_world_time for convenience
pub use saddle_world_time::{DayPhase, DayPhaseBoundaries, TimeOfDay, TimeStep, TimeStepMode};

use bevy::{
    app::PostStartup,
    ecs::{intern::Interned, schedule::ScheduleLabel},
    prelude::*,
};

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LightingSystems {
    ResolveCelestial,
    ResolveLighting,
    ApplyLighting,
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct NeverDeactivateSchedule;

pub struct LightingPlugin {
    pub activate_schedule: Interned<dyn ScheduleLabel>,
    pub deactivate_schedule: Interned<dyn ScheduleLabel>,
    pub update_schedule: Interned<dyn ScheduleLabel>,
    pub config: LightingConfig,
}

impl LightingPlugin {
    pub fn new(
        activate_schedule: impl ScheduleLabel,
        deactivate_schedule: impl ScheduleLabel,
        update_schedule: impl ScheduleLabel,
    ) -> Self {
        Self {
            activate_schedule: activate_schedule.intern(),
            deactivate_schedule: deactivate_schedule.intern(),
            update_schedule: update_schedule.intern(),
            config: LightingConfig::default(),
        }
    }

    pub fn always_on(update_schedule: impl ScheduleLabel) -> Self {
        Self::new(PostStartup, NeverDeactivateSchedule, update_schedule)
    }

    pub fn with_config(mut self, config: LightingConfig) -> Self {
        self.config = config;
        self
    }
}

impl Default for LightingPlugin {
    fn default() -> Self {
        Self::always_on(Update)
    }
}

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        if self.deactivate_schedule == NeverDeactivateSchedule.intern() {
            app.init_schedule(NeverDeactivateSchedule);
        }

        app.insert_resource(self.config.clone())
            .init_resource::<CelestialState>()
            .init_resource::<DayNightLighting>()
            .init_resource::<DayNightDiagnostics>()
            .init_resource::<WeatherModulation>()
            .init_resource::<systems::AtmosphereAssetCache>()
            .init_resource::<systems::LightingRuntimeState>()
            .init_resource::<GlobalAmbientLight>()
            .register_type::<AtmosphereTuning>()
            .register_type::<CelestialModel>()
            .register_type::<CelestialSettings>()
            .register_type::<CelestialState>()
            .register_type::<ColorGradient>()
            .register_type::<ColorKeyframe>()
            .register_type::<DayNightDiagnostics>()
            .register_type::<DayNightLighting>()
            .register_type::<LightingCamera>()
            .register_type::<LightingConfig>()
            .register_type::<LightingProfile>()
            .register_type::<ManagedLightConfig>()
            .register_type::<Moon>()
            .register_type::<MoonPhase>()
            .register_type::<ScalarGradient>()
            .register_type::<ScalarKeyframe>()
            .register_type::<SeasonSettings>()
            .register_type::<ShadowConfig>()
            .register_type::<SmoothingConfig>()
            .register_type::<Sun>()
            .register_type::<WeatherModulation>()
            .register_type::<WriteThresholds>()
            .add_systems(self.activate_schedule, systems::activate_runtime)
            .add_systems(self.deactivate_schedule, systems::deactivate_runtime)
            .configure_sets(
                self.update_schedule,
                (
                    LightingSystems::ResolveCelestial
                        .after(saddle_world_time::TimeSystems::AdvanceTime),
                    LightingSystems::ResolveLighting,
                    LightingSystems::ApplyLighting,
                )
                    .chain(),
            )
            .add_systems(
                self.update_schedule,
                systems::resolve_celestial_state
                    .in_set(LightingSystems::ResolveCelestial)
                    .run_if(systems::runtime_is_active),
            )
            .add_systems(
                self.update_schedule,
                systems::resolve_lighting_state
                    .in_set(LightingSystems::ResolveLighting)
                    .run_if(systems::runtime_is_active),
            )
            .add_systems(
                self.update_schedule,
                (
                    systems::ensure_managed_lights,
                    systems::apply_managed_sun,
                    systems::apply_managed_moon,
                    systems::apply_global_ambient_and_cameras,
                    systems::publish_diagnostics,
                )
                    .chain()
                    .in_set(LightingSystems::ApplyLighting)
                    .run_if(systems::runtime_is_active),
            );
    }
}
