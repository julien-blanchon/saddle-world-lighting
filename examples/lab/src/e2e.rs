use bevy::prelude::*;
use saddle_world_lighting::LightingSystems;

pub struct E2EPlugin;

impl Plugin for E2EPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "e2e")]
        {
            app.add_plugins(saddle_bevy_e2e::E2EPlugin);
            app.configure_sets(
                Update,
                saddle_bevy_e2e::E2ESet.after(LightingSystems::ApplyLighting),
            );

            if let Some(scenario) = parse_e2e_args() {
                saddle_bevy_e2e::init_scenario(app, scenario);
            }
        }
    }
}

#[cfg(feature = "e2e")]
fn parse_e2e_args() -> Option<saddle_bevy_e2e::scenario::Scenario> {
    let args: Vec<String> = std::env::args().collect();
    let scenario_name = args.iter().find(|arg| !arg.starts_with('-') && *arg != args[0]);
    let handoff = args.iter().any(|arg| arg == "--handoff");

    if let Some(name) = scenario_name {
        if let Some(scenario) = crate::scenarios::scenario_by_name(name) {
            info!("E2E scenario: {}", name);
            return Some(scenario);
        }
        error!(
            "Unknown scenario '{}'. Available: {:?}",
            name,
            crate::scenarios::list_scenarios()
        );
    } else if handoff {
        info!("Handoff mode \u{2014} no scenario selected");
    }

    None
}
