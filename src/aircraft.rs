use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor, time::common_conditions::on_timer};

use crate::{
    constant::{
        FEET_TO_METERS, KNOTS_TO_MPS, METERS_TO_FEET, MPS_TO_KNOTS, NAUTICAL_MILES_TO_METERS,
    },
    SimulationSettings,
};

/// A plugin for aircraft.
pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AircraftEvent>()
            .add_systems(PreUpdate, (process_aircraft_events,))
            .add_systems(
                FixedUpdate,
                (update_aircraft_position_fixed.run_if(on_timer(Duration::from_secs(1))),),
            )
            .add_systems(
                Update,
                (
                    update_aircraft_transform,
                    update_flight_number_labels,
                    update_altitude_labels,
                    update_ground_speed_labels,
                ),
            );
    }
}

/// An aircraft event.
#[derive(Debug, Event)]
pub enum AircraftEvent {
    /// Spawn an aircraft.
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
pub struct Aircraft;

#[derive(Debug, Default, Component)]
#[require(Aircraft)]
pub struct FlightNumber(pub String);

/// Position in 3D space in meters.
#[derive(Debug, Default, Component, Clone, Copy)]
pub struct Position(Vec3);

impl From<Vec3> for Position {
    fn from(vec: Vec3) -> Self {
        Self(vec)
    }
}

/// Velocity in 3D space in meters per second.
#[derive(Debug, Default, Component, Clone, Copy)]
pub struct Velocity(Vec3);

/// The flight number label.
#[derive(Debug, Default, Component)]
#[require(Text2d)]
pub struct FlightNumberLabel;

/// The altitude label.
#[derive(Debug, Default, Component)]
#[require(Text2d)]
pub struct AltitudeLabel;

/// The ground speed label.
#[derive(Debug, Default, Component)]
#[require(Text2d)]
pub struct GroundSpeedLabel;

fn update_aircraft_position_fixed(mut query: Query<(&Velocity, &mut Position), With<Aircraft>>) {
    query.par_iter_mut().for_each(|(velocity, mut position)| {
        position.0 += velocity.0;
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
            0.0,
        );
    });
}

fn process_aircraft_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

                info!(
                    "Spawn aircraft {}, FL{:0>3}, {:0>3.0} knots",
                    flight_number,
                    altitude / 100.0,
                    ground_speed.length()
                );

                commands
                    .spawn((
                        Aircraft,
                        FlightNumber(flight_number.clone()),
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
                    // flight number label
                    .with_child((
                        FlightNumberLabel,
                        Text2d::new(flight_number),
                        TextFont::from_font(font.clone()),
                        Transform::from_xyz(16.0, 0.0, 0.0),
                        Anchor::CenterLeft,
                    ))
                    .with_child((
                        AltitudeLabel,
                        Text2d::new(format!("FL{:0>3.0}", altitude / 100.0)),
                        TextFont::from_font(font.clone()),
                        Transform::from_xyz(16.0, -16.0, 0.0),
                        Anchor::CenterLeft,
                    ))
                    .with_child((
                        GroundSpeedLabel,
                        Text2d::new(format!("{:0>3.0}", ground_speed.length())),
                        TextFont::from_font(font.clone()),
                        Transform::from_xyz(16.0, -32.0, 0.0),
                        Anchor::CenterLeft,
                    ));
            }
        }
    }
}

fn update_flight_number_labels(
    mut q_labels: Query<(&Parent, &mut Text2d), With<FlightNumberLabel>>,
    q_parent: Query<&FlightNumber, With<Aircraft>>,
) {
    q_labels.par_iter_mut().for_each(|(parent, mut text)| {
        if let Ok(flight_number) = q_parent.get(parent.get()) {
            text.0 = flight_number.0.clone();
        }
    });
}

fn update_altitude_labels(
    mut q_labels: Query<(&Parent, &mut Text2d), With<AltitudeLabel>>,
    q_parent: Query<&Position, With<Aircraft>>,
) {
    q_labels.par_iter_mut().for_each(|(parent, mut text)| {
        if let Ok(position) = q_parent.get(parent.get()) {
            text.0 = format!("FL{:0>3.0}", position.0.z * METERS_TO_FEET / 100.0);
        }
    });
}

fn update_ground_speed_labels(
    mut q_labels: Query<(&Parent, &mut Text2d), With<GroundSpeedLabel>>,
    q_parent: Query<&Velocity, With<Aircraft>>,
) {
    q_labels.par_iter_mut().for_each(|(parent, mut text)| {
        if let Ok(velocity) = q_parent.get(parent.get()) {
            text.0 = format!("{:0>3.0}", velocity.0.xy().length() * MPS_TO_KNOTS);
        }
    });
}
