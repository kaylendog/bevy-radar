use bevy::prelude::*;

use crate::{
    constant::{FEET_TO_METERS, KNOTS_TO_MPS, NAUTICAL_MILES_TO_METERS},
    SimulationSettings,
};

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AircraftEvent>()
            .add_systems(PreUpdate, (process_aircraft_events,))
            .add_systems(FixedUpdate, (update_aircraft_position_fixed,))
            .add_systems(Update, (update_aircraft_transform,));
    }
}

#[derive(Debug, Default, Resource)]
pub struct AircraftAssets {
    pub icon: Handle<Image>,
    pub font: Handle<Font>,
}

#[derive(Debug, Event)]
pub enum AircraftEvent {
    Spawn {
        flight_number: String,
        position: Vec2,
        ground_speed: Vec2,
        altitude: f32,
    },
}

/// An aircraft.
#[derive(Debug, Default, Component)]
#[require(Position, Velocity, Transform, Sprite)]
struct Aircraft;

/// Position in 3D space in meters.
#[derive(Debug, Default, Component, Clone, Copy)]
struct Position(Vec3);

impl From<Vec3> for Position {
    fn from(vec: Vec3) -> Self {
        Self(vec)
    }
}

/// Velocity in 3D space in meters per second.
#[derive(Debug, Default, Component, Clone, Copy)]
struct Velocity(Vec3);

fn update_aircraft_position_fixed(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Position), With<Aircraft>>,
) {
    query.par_iter_mut().for_each(|(velocity, mut position)| {
        position.0 += velocity.0 * time.delta_secs();
    });
}

fn update_aircraft_transform(
    settings: Res<SimulationSettings>,
    mut query: Query<(&Position, &mut Transform), With<Aircraft>>,
) {
    query.par_iter_mut().for_each(|(position, mut transform)| {
        transform.translation = Vec3::new(
            position.0.x * settings.horizontal_scale,
            position.0.y * settings.horizontal_scale,
            position.0.z,
        );
    });
}

fn process_aircraft_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<SimulationSettings>,
    mut events: EventReader<AircraftEvent>,
) {
    for ev in events.read() {
        match ev {
            AircraftEvent::Spawn {
                flight_number,
                position,
                ground_speed,
                altitude,
            } => {
                let image = asset_server.load("sprites/aircraft/icon.png");
                let font = asset_server.load("fonts/NotoSansMono-VariableFont_wdth,wght.ttf");

                commands
                    .spawn((
                        Aircraft,
                        Position(Vec3::new(
                            position.x * NAUTICAL_MILES_TO_METERS,
                            position.y * NAUTICAL_MILES_TO_METERS,
                            *altitude * FEET_TO_METERS,
                        )),
                        Velocity(Vec3::new(
                            ground_speed.x * KNOTS_TO_MPS,
                            ground_speed.y * KNOTS_TO_MPS,
                            0.0,
                        )),
                        Sprite::from_image(image),
                    ))
                    // aircraft label
                    .with_child((
                        Text2d::new(flight_number),
                        TextFont::from_font(font),
                        Transform::from_xyz(48.0, 0.0, 0.0),
                    ));
            }
        }
    }
}
