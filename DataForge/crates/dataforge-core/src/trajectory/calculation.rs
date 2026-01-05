//! Trajectory Calculation Module
//!
//! Implements minimum curvature and other trajectory calculation methods.
//! Used to calculate TVD, NS, EW, and DLS from MD, inclination, and azimuth.

use std::f64::consts::PI;

/// Tie-in point for trajectory calculations
///
/// Represents the starting point for trajectory calculations.
/// All calculated values (TVD, NS, EW) are offsets from this point.
#[derive(Debug, Clone, Default)]
pub struct TieInPoint {
    pub md: f64,
    pub tvd: f64,
    pub ns: f64,
    pub ew: f64,
    pub inclination: f64,
    pub azimuth: f64,
}

/// Result of a trajectory calculation for a single station
#[derive(Debug, Clone)]
pub struct CalculatedStation {
    /// Measured depth
    pub md: f64,
    /// Inclination (degrees)
    pub inclination: f64,
    /// Azimuth (degrees)
    pub azimuth: f64,
    /// True vertical depth
    pub tvd: f64,
    /// North-South offset (positive = north)
    pub ns: f64,
    /// East-West offset (positive = east)
    pub ew: f64,
    /// Dogleg severity (degrees per 100ft or per 30m)
    pub dls: f64,
    /// Vertical section
    pub vs: f64,
}

/// Result of calculating an entire trajectory
#[derive(Debug, Clone)]
pub struct TrajectoryCalculationResult {
    pub stations: Vec<CalculatedStation>,
    pub tie_in: TieInPoint,
    pub total_md: f64,
    pub total_tvd: f64,
    pub max_inclination: f64,
    pub max_dls: f64,
}

/// Input station data for calculation
#[derive(Debug, Clone)]
pub struct InputStation {
    pub md: f64,
    pub inclination: f64, // degrees
    pub azimuth: f64,     // degrees
}

/// Calculate trajectory using minimum curvature method
///
/// This is the industry-standard method for calculating well trajectories.
/// It assumes the wellbore follows a smooth arc between survey stations.
///
/// # Arguments
/// * `stations` - Input stations with MD, inclination, and azimuth
/// * `tie_in` - Optional tie-in point (defaults to origin)
/// * `dls_unit_length` - Length for DLS normalization (100 for ft, 30 for m)
///
/// # Returns
/// Calculated stations with TVD, NS, EW, DLS values
pub fn calculate_trajectory(
    stations: &[InputStation],
    tie_in: Option<TieInPoint>,
    dls_unit_length: f64,
) -> TrajectoryCalculationResult {
    let tie_in = tie_in.unwrap_or_default();

    if stations.is_empty() {
        return TrajectoryCalculationResult {
            stations: Vec::new(),
            tie_in: tie_in.clone(),
            total_md: 0.0,
            total_tvd: 0.0,
            max_inclination: 0.0,
            max_dls: 0.0,
        };
    }

    let mut calculated: Vec<CalculatedStation> = Vec::with_capacity(stations.len());
    let mut current_tvd = tie_in.tvd;
    let mut current_ns = tie_in.ns;
    let mut current_ew = tie_in.ew;
    let mut prev_inc = tie_in.inclination;
    let mut prev_azi = tie_in.azimuth;
    let mut prev_md = tie_in.md;
    let mut max_inc = 0.0_f64;
    let mut max_dls = 0.0_f64;

    for station in stations {
        let course_length = station.md - prev_md;

        // Calculate deltas using minimum curvature
        let (delta_tvd, delta_ns, delta_ew) = if course_length > 0.0 {
            minimum_curvature(
                prev_inc,
                prev_azi,
                station.inclination,
                station.azimuth,
                course_length,
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        // Calculate dogleg severity
        let dls = if course_length > 0.0 {
            calculate_dls(
                prev_inc,
                prev_azi,
                station.inclination,
                station.azimuth,
                course_length,
                dls_unit_length,
            )
        } else {
            0.0
        };

        current_tvd += delta_tvd;
        current_ns += delta_ns;
        current_ew += delta_ew;

        // Calculate vertical section (horizontal displacement from origin)
        let vs = (current_ns.powi(2) + current_ew.powi(2)).sqrt();

        max_inc = max_inc.max(station.inclination);
        max_dls = max_dls.max(dls);

        calculated.push(CalculatedStation {
            md: station.md,
            inclination: station.inclination,
            azimuth: station.azimuth,
            tvd: current_tvd,
            ns: current_ns,
            ew: current_ew,
            dls,
            vs,
        });

        prev_md = station.md;
        prev_inc = station.inclination;
        prev_azi = station.azimuth;
    }

    let total_md = stations.last().map(|s| s.md).unwrap_or(0.0) - tie_in.md;
    let total_tvd = current_tvd - tie_in.tvd;

    TrajectoryCalculationResult {
        stations: calculated,
        tie_in,
        total_md,
        total_tvd,
        max_inclination: max_inc,
        max_dls,
    }
}

/// Minimum curvature calculation for a single interval
///
/// Calculates the change in TVD, NS, and EW between two survey stations.
///
/// # Arguments
/// * `inc1` - Inclination at start (degrees)
/// * `azi1` - Azimuth at start (degrees)
/// * `inc2` - Inclination at end (degrees)
/// * `azi2` - Azimuth at end (degrees)
/// * `course_length` - Measured depth interval
///
/// # Returns
/// Tuple of (delta_tvd, delta_ns, delta_ew)
pub fn minimum_curvature(
    inc1: f64,
    azi1: f64,
    inc2: f64,
    azi2: f64,
    course_length: f64,
) -> (f64, f64, f64) {
    // Convert to radians
    let i1 = inc1.to_radians();
    let a1 = azi1.to_radians();
    let i2 = inc2.to_radians();
    let a2 = azi2.to_radians();

    // Calculate dogleg angle
    let dogleg = calculate_dogleg_radians(i1, a1, i2, a2);

    // Calculate ratio factor (RF)
    // RF = 2/dogleg * tan(dogleg/2) for dogleg > 0
    // RF = 1 for dogleg ≈ 0 (straight section)
    let rf = if dogleg.abs() < 1e-7 {
        1.0
    } else {
        2.0 / dogleg * (dogleg / 2.0).tan()
    };

    // Calculate position changes
    let half_cl = course_length / 2.0;

    let delta_tvd = half_cl * (i1.cos() + i2.cos()) * rf;
    let delta_ns = half_cl * (i1.sin() * a1.cos() + i2.sin() * a2.cos()) * rf;
    let delta_ew = half_cl * (i1.sin() * a1.sin() + i2.sin() * a2.sin()) * rf;

    (delta_tvd, delta_ns, delta_ew)
}

/// Calculate dogleg severity (DLS)
///
/// DLS represents the rate of wellbore curvature, typically expressed
/// as degrees per 100 feet or degrees per 30 meters.
///
/// # Arguments
/// * `inc1` - Inclination at start (degrees)
/// * `azi1` - Azimuth at start (degrees)
/// * `inc2` - Inclination at end (degrees)
/// * `azi2` - Azimuth at end (degrees)
/// * `course_length` - Measured depth interval
/// * `unit_length` - Normalization length (100 for ft, 30 for m)
///
/// # Returns
/// Dogleg severity in degrees per unit length
pub fn calculate_dls(
    inc1: f64,
    azi1: f64,
    inc2: f64,
    azi2: f64,
    course_length: f64,
    unit_length: f64,
) -> f64 {
    if course_length.abs() < 1e-10 {
        return 0.0;
    }

    let i1 = inc1.to_radians();
    let a1 = azi1.to_radians();
    let i2 = inc2.to_radians();
    let a2 = azi2.to_radians();

    let dogleg = calculate_dogleg_radians(i1, a1, i2, a2);

    // Convert to degrees and normalize to unit length
    (dogleg * 180.0 / PI) * (unit_length / course_length)
}

/// Calculate dogleg angle in radians
///
/// Uses the spherical law of cosines formula.
fn calculate_dogleg_radians(inc1: f64, azi1: f64, inc2: f64, azi2: f64) -> f64 {
    // Spherical law of cosines for dogleg
    let cos_dogleg = inc1.cos() * inc2.cos() + inc1.sin() * inc2.sin() * (azi2 - azi1).cos();

    // Clamp to [-1, 1] to handle floating point errors
    cos_dogleg.clamp(-1.0, 1.0).acos()
}

/// Calculate vertical section along a given azimuth
///
/// Projects the horizontal displacement onto a vertical plane
/// oriented along the specified azimuth.
///
/// # Arguments
/// * `ns` - North-South offset
/// * `ew` - East-West offset
/// * `vs_azimuth` - Azimuth of vertical section plane (degrees)
pub fn calculate_vertical_section(ns: f64, ew: f64, vs_azimuth: f64) -> f64 {
    let azi_rad = vs_azimuth.to_radians();
    ns * azi_rad.cos() + ew * azi_rad.sin()
}

/// Convert angles from one unit to another
pub fn convert_angle(value: f64, from_unit: &str, to_unit: &str) -> f64 {
    if from_unit == to_unit {
        return value;
    }

    // Convert to radians first
    let radians = match from_unit {
        "deg" | "degrees" => value.to_radians(),
        "rad" | "radians" => value,
        "grad" | "gradians" => value * PI / 200.0,
        _ => value.to_radians(), // Default assume degrees
    };

    // Convert from radians to target
    match to_unit {
        "deg" | "degrees" => radians.to_degrees(),
        "rad" | "radians" => radians,
        "grad" | "gradians" => radians * 200.0 / PI,
        _ => radians.to_degrees(), // Default to degrees
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.01; // Allow 0.01 tolerance for calculations

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn test_vertical_well() {
        // Perfectly vertical well - no inclination
        let (delta_tvd, delta_ns, delta_ew) = minimum_curvature(0.0, 0.0, 0.0, 0.0, 100.0);

        assert!(approx_eq(delta_tvd, 100.0, EPSILON), "TVD should equal MD for vertical well");
        assert!(approx_eq(delta_ns, 0.0, EPSILON), "NS should be 0 for vertical well");
        assert!(approx_eq(delta_ew, 0.0, EPSILON), "EW should be 0 for vertical well");
    }

    #[test]
    fn test_horizontal_well_north() {
        // Horizontal well going north (inc=90, azi=0)
        let (delta_tvd, delta_ns, delta_ew) = minimum_curvature(90.0, 0.0, 90.0, 0.0, 100.0);

        assert!(approx_eq(delta_tvd, 0.0, EPSILON), "TVD change should be ~0 for horizontal");
        assert!(approx_eq(delta_ns, 100.0, EPSILON), "NS should equal MD for north-heading horizontal");
        assert!(approx_eq(delta_ew, 0.0, EPSILON), "EW should be 0 for north-heading");
    }

    #[test]
    fn test_horizontal_well_east() {
        // Horizontal well going east (inc=90, azi=90)
        let (delta_tvd, delta_ns, delta_ew) = minimum_curvature(90.0, 90.0, 90.0, 90.0, 100.0);

        assert!(approx_eq(delta_tvd, 0.0, EPSILON));
        assert!(approx_eq(delta_ns, 0.0, EPSILON));
        assert!(approx_eq(delta_ew, 100.0, EPSILON));
    }

    #[test]
    fn test_build_section() {
        // Building from vertical to 45 degrees, heading north
        let (delta_tvd, delta_ns, delta_ew) = minimum_curvature(0.0, 0.0, 45.0, 0.0, 100.0);

        // Expected values for this case (approximations from industry standard)
        // TVD should be less than 100 (we're building angle)
        // NS should be positive (heading north)
        assert!(delta_tvd > 0.0 && delta_tvd < 100.0);
        assert!(delta_ns > 0.0);
        assert!(approx_eq(delta_ew, 0.0, EPSILON));
    }

    #[test]
    fn test_dls_calculation() {
        // 3 degree inclination change over 100ft = 3 deg/100ft DLS
        let dls = calculate_dls(0.0, 0.0, 3.0, 0.0, 100.0, 100.0);
        assert!(approx_eq(dls, 3.0, 0.1));

        // Same change over 50ft = 6 deg/100ft DLS
        let dls = calculate_dls(0.0, 0.0, 3.0, 0.0, 50.0, 100.0);
        assert!(approx_eq(dls, 6.0, 0.1));
    }

    #[test]
    fn test_dls_azimuth_change() {
        // Azimuth change at inclination affects DLS
        // At 45 deg inclination, 10 deg azimuth change over 100ft
        let dls = calculate_dls(45.0, 0.0, 45.0, 10.0, 100.0, 100.0);
        assert!(dls > 0.0);
        assert!(dls < 10.0); // Should be less than azimuth change due to geometry
    }

    #[test]
    fn test_trajectory_calculation() {
        let stations = vec![
            InputStation { md: 0.0, inclination: 0.0, azimuth: 0.0 },
            InputStation { md: 100.0, inclination: 0.0, azimuth: 0.0 },
            InputStation { md: 200.0, inclination: 15.0, azimuth: 45.0 },
            InputStation { md: 300.0, inclination: 30.0, azimuth: 45.0 },
        ];

        let result = calculate_trajectory(&stations, None, 100.0);

        assert_eq!(result.stations.len(), 4);

        // First station should have initial values
        assert!(approx_eq(result.stations[0].md, 0.0, EPSILON));
        assert!(approx_eq(result.stations[0].tvd, 0.0, EPSILON));
        assert!(approx_eq(result.stations[0].dls, 0.0, EPSILON));

        // Second station - still vertical
        assert!(approx_eq(result.stations[1].tvd, 100.0, EPSILON));

        // Subsequent stations should have increasing TVD (less than MD due to angle)
        assert!(result.stations[3].tvd < result.stations[3].md);

        // Max inclination should be 30
        assert!(approx_eq(result.max_inclination, 30.0, EPSILON));
    }

    #[test]
    fn test_trajectory_with_tie_in() {
        let stations = vec![
            InputStation { md: 1000.0, inclination: 5.0, azimuth: 45.0 },
            InputStation { md: 1100.0, inclination: 10.0, azimuth: 45.0 },
        ];

        let tie_in = TieInPoint {
            md: 1000.0,
            tvd: 990.0,
            ns: 50.0,
            ew: 30.0,
            inclination: 5.0,
            azimuth: 45.0,
        };

        let result = calculate_trajectory(&stations, Some(tie_in.clone()), 100.0);

        // First station should match tie-in (zero deltas)
        assert!(approx_eq(result.stations[0].tvd, tie_in.tvd, EPSILON));
        assert!(approx_eq(result.stations[0].ns, tie_in.ns, EPSILON));
        assert!(approx_eq(result.stations[0].ew, tie_in.ew, EPSILON));
    }

    #[test]
    fn test_vertical_section() {
        // NS = 100, EW = 0, VS azimuth = 0 (north) -> VS = 100
        assert!(approx_eq(calculate_vertical_section(100.0, 0.0, 0.0), 100.0, EPSILON));

        // NS = 0, EW = 100, VS azimuth = 90 (east) -> VS = 100
        assert!(approx_eq(calculate_vertical_section(0.0, 100.0, 90.0), 100.0, EPSILON));

        // NS = 100, EW = 100, VS azimuth = 45 -> VS = 141.4
        let vs = calculate_vertical_section(100.0, 100.0, 45.0);
        assert!(approx_eq(vs, 141.42, 0.1));
    }

    #[test]
    fn test_angle_conversion() {
        assert!(approx_eq(convert_angle(180.0, "deg", "rad"), PI, 0.001));
        assert!(approx_eq(convert_angle(PI, "rad", "deg"), 180.0, 0.001));
        assert!(approx_eq(convert_angle(90.0, "deg", "grad"), 100.0, 0.001));
        assert!(approx_eq(convert_angle(45.0, "deg", "deg"), 45.0, 0.001));
    }

    #[test]
    fn test_empty_trajectory() {
        let result = calculate_trajectory(&[], None, 100.0);
        assert!(result.stations.is_empty());
        assert!(approx_eq(result.total_md, 0.0, EPSILON));
    }

    #[test]
    fn test_dogleg_zero_inclination() {
        // At zero inclination, azimuth change shouldn't create dogleg
        let dogleg = calculate_dogleg_radians(0.0, 0.0, 0.0, PI);
        assert!(approx_eq(dogleg, 0.0, 0.001));
    }
}
