/// Convert feet to meters.
pub const FEET_TO_METERS: f32 = 0.3048;

/// Convert meters to feet.
pub const METERS_TO_FEET: f32 = 1.0 / FEET_TO_METERS;

/// Convert meters to nautical miles.
pub const METERS_TO_NAUTICAL_MILES: f32 = 0.000539957;

/// Convert nautical miles to meters.
pub const NAUTICAL_MILES_TO_METERS: f32 = 1.0 / METERS_TO_NAUTICAL_MILES;

/// Convert meters per second to feet per minute.
pub const MPS_TO_FPM: f32 = METERS_TO_FEET * 60.0;

/// Convert feet per minute to meters per second.
pub const FPM_TO_MPS: f32 = 1.0 / MPS_TO_FPM;

/// Convert MPS to knots.
pub const MPS_TO_KNOTS: f32 = METERS_TO_NAUTICAL_MILES * 3600.0;

/// Convert knots to MPS.
pub const KNOTS_TO_MPS: f32 = 1.0 / MPS_TO_KNOTS;
