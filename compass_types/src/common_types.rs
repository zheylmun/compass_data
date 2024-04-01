const FEET_TO_METERS: f64 = 0.3048;
const METERS_TO_FEET: f64 = 1.0 / FEET_TO_METERS;

/// East North Up coordinates
/// Always stored in meters
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EastNorthUp {
    pub east: f64,
    pub north: f64,
    pub up: f64,
}

impl EastNorthUp {
    pub fn from_meters(east: f64, north: f64, up: f64) -> Self {
        Self { east, north, up }
    }

    pub fn from_feet(east: f64, north: f64, up: f64) -> Self {
        Self {
            east: east * FEET_TO_METERS,
            north: north * FEET_TO_METERS,
            up: up * FEET_TO_METERS,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UtmLocation {
    pub east_north_elevation: EastNorthUp,
    pub zone: u8,
    pub convergence_angle: f64,
}
