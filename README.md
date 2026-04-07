# Saddle World Lighting

Celestial lighting, sun/moon management, fog, exposure, and atmosphere driven by time-of-day for Bevy.

Depends on [`saddle-world-time`](https://github.com/julien-blanchon/saddle-world-time) for the clock — this crate only owns the visual/lighting response.

## Quick Start

```rust
use bevy::prelude::*;
use saddle_world_time::{TimeConfig, TimePlugin};
use saddle_world_lighting::{LightingConfig, LightingPlugin, LightingProfile};

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(TimePlugin::new(
        OnEnter(MyState::Gameplay), OnExit(MyState::Gameplay), Update,
    ).with_config(TimeConfig {
        initial_time: 7.5,
        seconds_per_hour: 120.0,
        ..default()
    }))
    .add_plugins(LightingPlugin::new(
        OnEnter(MyState::Gameplay), OnExit(MyState::Gameplay), Update,
    ).with_config(LightingConfig::default().with_profile(
        LightingProfile::realistic_outdoor(),
    )))
    .run();
```

## Public API

Plugins:

- `LightingPlugin` — celestial solving, lighting resolution, managed lights, fog, exposure, atmosphere

System sets (chained):

- `LightingSystems::ResolveCelestial` (runs after `TimeSystems::AdvanceTime`)
- `LightingSystems::ResolveLighting`
- `LightingSystems::ApplyLighting`

Resources:

- `LightingConfig` — celestial model, lighting profile, shadows, smoothing, atmosphere
- `CelestialState` — resolved sun/moon positions, daylight factors, moon phase
- `DayNightLighting` — resolved colors, illuminance, fog, exposure
- `DayNightDiagnostics` — write counters and debug state
- `WeatherModulation` — input for cloud/haze/precipitation dimming

Components:

- `Sun` — marker for managed sun directional light
- `Moon` — marker for managed moon directional light
- `LightingCamera` — marks cameras for fog, exposure, and atmosphere management

Lighting profiles (presets):

- `LightingProfile::realistic_outdoor()`
- `LightingProfile::stylized_saturated()`
- `LightingProfile::overcast()`
- `LightingProfile::harsh_desert()`
- `LightingProfile::moonlit_night()`

Re-exports from `saddle-world-time`:

- `TimeOfDay`, `DayPhase`, `DayPhaseBoundaries`, `TimeSystems`

## Examples

| Example | Purpose |
|---------|---------|
| `basic` | Minimal sun/moon setup with outdoor scene |
| `full_cycle` | Fast 24h cycle showing dawn→day→dusk→night |
| `latitude` | Latitude-aware sun path with seasonal variation |
| `fixed_time` | Paused golden-hour art direction |
| `street_lights` | TimeReactive lamps using the time crate |
| `lab` | E2E scenarios for smoke testing and quality verification |

## E2E Scenarios

```bash
cargo run -p saddle-world-lighting-lab --features e2e -- lighting_smoke
cargo run -p saddle-world-lighting-lab --features e2e -- lighting_full_cycle
```

## Documentation

- [`docs/architecture.md`](docs/architecture.md)
- [`docs/configuration.md`](docs/configuration.md)
