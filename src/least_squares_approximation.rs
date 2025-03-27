use std::fs::File;
use std::io::Write;

/// Performs least squares approximation on the temperature data and writes results to files.
///
/// # Arguments
/// * `times` - A vector of time steps.
/// * `readings_core` - A vector of vectors containing temeperature readings for a specific core.
/// * `file` - The file to write output to.
pub fn least_squares_approximation(times: &Vec<f64>, readings_core: &Vec<f64>, mut file: &File) {
    let k = times.len();

    let mut sum_x = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;

    for i in 0..k {
        sum_x += times[i];
        sum_y += readings_core[i];
        sum_x2 += times[i] * times[i];
        sum_xy += times[i] * readings_core[i];
    }

    let det = k as f64 * sum_x2 - sum_x * sum_x;

    let b = (sum_y * sum_x2 - sum_x * sum_xy) / det;
    let m = (k as f64 * sum_xy - sum_y * sum_x) / det;

    writeln!(
        file,
        "{:>6} <= x <= {:>6} ; y = {:>10.4} + {:>10.4} x ; least-squares",
        times[0],
        times[k - 1],
        b,
        m
    )
    .expect("Failed to write to file");
}
