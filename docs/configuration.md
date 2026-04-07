# Configuration

## LightingConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `celestial` | `CelestialSettings` | See below | Sun/moon path model |
| `lighting` | `LightingProfile` | `realistic_outdoor()` | Gradient curves for illuminance, color temperature, fog |
| `managed_lights` | `ManagedLightConfig` | `auto_spawn: true` | Auto-spawn Sun/Moon if missing |
| `shadows` | `ShadowConfig` | See below | Min elevation/illuminance for shadow casting |
| `smoothing` | `SmoothingConfig` | See below | Temporal interpolation |
| `write_thresholds` | `WriteThresholds` | See below | Epsilon values to avoid redundant writes |
| `atmosphere` | `AtmosphereTuning` | See below | Atmosphere rendering parameters |

## CelestialSettings

| Field | Default | Description |
|-------|---------|-------------|
| `model` | `SimpleArc { peak: 72° }` | Sun path model |
| `azimuth_offset_degrees` | `0.0` | Rotate sun path around horizon |
| `moon_hour_offset` | `12.0` | Moon offset from sun |
| `moon_elevation_offset_degrees` | `8.0` | Extra moon height |
| `lunar_period_days` | `29.53` | Synodic period |
| `lunar_phase_offset` | `0.0` | Phase at time zero |

## LightingCamera

| Field | Default | Description |
|-------|---------|-------------|
| `enabled` | `true` | Enable lighting management |
| `apply_distance_fog` | `true` | Write DistanceFog |
| `apply_volumetric_fog` | `true` | Write VolumetricFog |
| `apply_exposure` | `true` | Write Exposure |
| `apply_environment_map_light` | `true` | Write AtmosphereEnvironmentMapLight |
| `insert_missing_components` | `true` | Auto-insert missing components |
| `ensure_atmosphere` | `false` | Insert Atmosphere component |

## WeatherModulation

| Field | Default | Description |
|-------|---------|-------------|
| `cloud_cover` | `0.0` | 0-1, dims sun and reflection |
| `haze` | `0.0` | 0-1, reduces visibility |
| `precipitation_dimming` | `0.0` | 0-1, dims ambient and thickens fog |
