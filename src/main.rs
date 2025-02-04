use bevy::prelude::*;

mod aircraft;
mod camera;
mod constant;

use aircraft::{AircraftEvent, AircraftPlugin};
use camera::CameraPlugin;

/// The radar simulation settings.
#[derive(Debug, Resource)]
struct SimulationSettings {
    /// The ratio of simulation position to render position.
    horizontal_scale: f32,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            horizontal_scale: 0.1,
        }
    }
}

fn spawn_debug_aircraft(mut events: EventWriter<AircraftEvent>) {
    events.send(AircraftEvent::Spawn {
        flight_number: "12345".to_string(),
        position: Vec2::ZERO,
        ground_speed: Vec2::ZERO,
        altitude: 0.0,
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((AircraftPlugin, CameraPlugin))
        .init_resource::<SimulationSettings>()
        .add_systems(Startup, spawn_debug_aircraft)
        .run();
}
