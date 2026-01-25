const FEET_TO_METERS: f64 = 0.3048;

/// East North Elevation coordinates
/// Always stored in meters
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UtmLocation {
    pub east_north_elevation: EastNorthElevation,
    pub zone: u8,
    pub convergence_angle: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Date {
    pub month: u8,
    pub day: u8,
    pub year: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    // EastNorthElevation tests
    #[test]
    fn test_from_meters_basic() {
        let ene = EastNorthElevation::from_meters(100.0, 200.0, 300.0);
        assert_float_eq!(ene.easting, 100.0, abs <= 1e-10);
        assert_float_eq!(ene.northing, 200.0, abs <= 1e-10);
        assert_float_eq!(ene.up, 300.0, abs <= 1e-10);
    }

    #[test]
    fn test_from_meters_zero() {
        let ene = EastNorthElevation::from_meters(0.0, 0.0, 0.0);
        assert_float_eq!(ene.easting, 0.0, abs <= 1e-10);
        assert_float_eq!(ene.northing, 0.0, abs <= 1e-10);
        assert_float_eq!(ene.up, 0.0, abs <= 1e-10);
    }

    #[test]
    fn test_from_meters_negative() {
        let ene = EastNorthElevation::from_meters(-100.0, -200.0, -50.0);
        assert_float_eq!(ene.easting, -100.0, abs <= 1e-10);
        assert_float_eq!(ene.northing, -200.0, abs <= 1e-10);
        assert_float_eq!(ene.up, -50.0, abs <= 1e-10);
    }

    #[test]
    fn test_from_feet_basic() {
        // 1 foot = 0.3048 meters
        let ene = EastNorthElevation::from_feet(100.0, 200.0, 300.0);
        assert_float_eq!(ene.easting, 30.48, abs <= 1e-10);
        assert_float_eq!(ene.northing, 60.96, abs <= 1e-10);
        assert_float_eq!(ene.up, 91.44, abs <= 1e-10);
    }

    #[test]
    fn test_from_feet_zero() {
        let ene = EastNorthElevation::from_feet(0.0, 0.0, 0.0);
        assert_float_eq!(ene.easting, 0.0, abs <= 1e-10);
        assert_float_eq!(ene.northing, 0.0, abs <= 1e-10);
        assert_float_eq!(ene.up, 0.0, abs <= 1e-10);
    }

    #[test]
    fn test_from_feet_negative() {
        let ene = EastNorthElevation::from_feet(-10.0, -20.0, -30.0);
        assert_float_eq!(ene.easting, -3.048, abs <= 1e-10);
        assert_float_eq!(ene.northing, -6.096, abs <= 1e-10);
        assert_float_eq!(ene.up, -9.144, abs <= 1e-10);
    }

    #[test]
    fn test_from_feet_known_conversion() {
        // 1 foot exactly
        let ene = EastNorthElevation::from_feet(1.0, 1.0, 1.0);
        assert_float_eq!(ene.easting, FEET_TO_METERS, abs <= 1e-10);
        assert_float_eq!(ene.northing, FEET_TO_METERS, abs <= 1e-10);
        assert_float_eq!(ene.up, FEET_TO_METERS, abs <= 1e-10);
    }

    #[test]
    fn test_from_feet_large_values() {
        // Large coordinate values typical in UTM
        let ene = EastNorthElevation::from_feet(1_000_000.0, 4_000_000.0, 10_000.0);
        assert_float_eq!(ene.easting, 304_800.0, abs <= 1e-6);
        assert_float_eq!(ene.northing, 1_219_200.0, abs <= 1e-6);
        assert_float_eq!(ene.up, 3_048.0, abs <= 1e-6);
    }

    // UtmLocation tests
    #[test]
    fn test_utm_location_creation() {
        let ene = EastNorthElevation::from_meters(500_000.0, 4_500_000.0, 1500.0);
        let utm = UtmLocation {
            east_north_elevation: ene,
            zone: 13,
            convergence_angle: -0.5,
        };
        assert_eq!(utm.zone, 13);
        assert_float_eq!(utm.convergence_angle, -0.5, abs <= 1e-10);
        assert_float_eq!(utm.east_north_elevation.easting, 500_000.0, abs <= 1e-10);
    }

    #[test]
    fn test_utm_location_zone_boundaries() {
        let ene = EastNorthElevation::from_meters(0.0, 0.0, 0.0);

        // Zone 1 (minimum valid zone)
        let utm1 = UtmLocation {
            east_north_elevation: ene,
            zone: 1,
            convergence_angle: 0.0,
        };
        assert_eq!(utm1.zone, 1);

        // Zone 60 (maximum valid zone)
        let utm60 = UtmLocation {
            east_north_elevation: ene,
            zone: 60,
            convergence_angle: 0.0,
        };
        assert_eq!(utm60.zone, 60);
    }

    #[test]
    fn test_utm_location_convergence_angle_range() {
        let ene = EastNorthElevation::from_meters(500_000.0, 4_500_000.0, 0.0);

        // Positive convergence angle
        let utm_pos = UtmLocation {
            east_north_elevation: ene,
            zone: 13,
            convergence_angle: 2.5,
        };
        assert_float_eq!(utm_pos.convergence_angle, 2.5, abs <= 1e-10);

        // Negative convergence angle
        let utm_neg = UtmLocation {
            east_north_elevation: ene,
            zone: 13,
            convergence_angle: -2.5,
        };
        assert_float_eq!(utm_neg.convergence_angle, -2.5, abs <= 1e-10);
    }

    // Date tests
    #[test]
    fn test_date_creation() {
        let date = Date {
            month: 6,
            day: 15,
            year: 2024,
        };
        assert_eq!(date.month, 6);
        assert_eq!(date.day, 15);
        assert_eq!(date.year, 2024);
    }

    #[test]
    fn test_date_january_first() {
        let date = Date {
            month: 1,
            day: 1,
            year: 2000,
        };
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 1);
        assert_eq!(date.year, 2000);
    }

    #[test]
    fn test_date_december_last() {
        let date = Date {
            month: 12,
            day: 31,
            year: 1999,
        };
        assert_eq!(date.month, 12);
        assert_eq!(date.day, 31);
        assert_eq!(date.year, 1999);
    }

    #[test]
    fn test_date_leap_year() {
        let date = Date {
            month: 2,
            day: 29,
            year: 2024,
        };
        assert_eq!(date.month, 2);
        assert_eq!(date.day, 29);
        assert_eq!(date.year, 2024);
    }
}
