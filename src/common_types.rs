const FEET_TO_METERS: f64 = 0.3048;

/// East North Elevation coordinates
/// Always stored in meters
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EastNorthElevation {
    pub easting: f64,
    pub northing: f64,
    pub up: f64,
}

impl EastNorthElevation {
    #[must_use]
    pub fn from_meters(easting: f64, northing: f64, up: f64) -> Self {
        Self {
            easting,
            northing,
            up,
        }
    }

    #[must_use]
    pub fn from_feet(easting: f64, northing: f64, up: f64) -> Self {
        Self {
            easting: easting * FEET_TO_METERS,
            northing: northing * FEET_TO_METERS,
            up: up * FEET_TO_METERS,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UtmLocation {
    pub east_north_elevation: EastNorthElevation,
    pub zone: u8,
    pub convergence_angle: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Date {
    pub month: u8,
    pub day: u8,
    pub year: u16,
}
