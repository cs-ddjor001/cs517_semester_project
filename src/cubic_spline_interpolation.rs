use std::fs::File;
use std::io::Write;

/// Performs cubic spline interpolation on the temperature data and writes results to files.
///
/// # Arguments
/// * `times` - A vector of time steps.
/// * `readings_core` - A vector of vectors containing temeperature readings for a specific core.
/// * `file` - The file to write output to.
pub fn cubic_spline_interpolation(times: &Vec<f64>, readings_core: &Vec<f64>, mut file: &File) {
    let k = times.len();

    let mut delta_x = vec![0.0; k - 1];

    for i in 0..k-1 {
        delta_x[i] = times[i + 1] - times[i];
    }

    let c0 = 0;
    let c1 = 0;
    let c2 = 0;
    let c3 = 0;

    writeln!(
        file,
        "{:>6} <= x <= {:>6} ; y = {:>10.4} + {:>10.4} x + {:>10.4} x^2 + {:>10.4} x^3; cubic-spline interpolation",
        times[0],
        times[k - 1],
        c0,
        c1,
        c2,
        c3

    )
    .expect("Failed to write to file");
}
