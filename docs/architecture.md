# Architecture

## Overview

`saddle-world-lighting` provides outdoor lighting orchestration for Bevy. It reads `TimeOfDay` from `saddle-world-time` and resolves sun/moon positions, ambient light, fog, and exposure.

## Dependency

```
saddle-world-time (standalone, leaf crate)
       ^
saddle-world-lighting (reads TimeOfDay, writes CelestialState + DayNightLighting)
```

## System flow

```
TimeSystems::AdvanceTime (from saddle-world-time)
  |
  v
LightingSystems::ResolveCelestial
  → solve_celestial_state(TimeOfDay, CelestialSettings)
  → writes CelestialState

LightingSystems::ResolveLighting
  → resolve_lighting(TimeOfDay, CelestialState, LightingProfile, WeatherModulation)
  → smooth_lighting() for temporal interpolation
  → writes DayNightLighting

LightingSystems::ApplyLighting
  → ensure_managed_lights(): auto-spawn Sun/Moon if needed
  → apply_managed_sun(): direction, color, illuminance, shadows
  → apply_managed_moon(): same
  → apply_global_ambient_and_cameras(): ambient light, fog, exposure, atmosphere
  → publish_diagnostics()
```

## Ownership

- `Sun` / `Moon` markers: plugin writes DirectionalLight on these entities
- `LightingCamera` marker: plugin writes DistanceFog, VolumetricFog, Exposure, Atmosphere
- `GlobalAmbientLight`: driven by plugin while active
- `WeatherModulation`: written by consumers (e.g., weather crate sync)

## Weather integration

The `WeatherModulation` resource provides cloud_cover, haze, and precipitation_dimming inputs. The lighting resolver uses these to modulate sun illuminance, ambient brightness, fog density, and exposure. The weather crate remains independent — games compose them with a thin sync system.
