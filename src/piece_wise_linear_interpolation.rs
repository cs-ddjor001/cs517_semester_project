use std::fs::File;
use std::io::Write;

/// Performs piecewise linear interpolation on the temperature data and writes results to files.
///
/// # Arguments
/// * `base_file_name` - The base name for the output files.
/// * `times` - A vector of time steps.
/// * `readings_core_0` - A vector of vectors containing temeperature readings for core 0
/// * `readings_core_1` - A vector of vectors containing temeperature readings for core 1
/// * `readings_core_2` - A vector of vectors containing temeperature readings for core 2
/// * `readings_core_3` - A vector of vectors containing temeperature readings for core 3.
pub fn piece_wise_linear_interpolation(
    times: &Vec<f64>,
    readings_core: Vec<f64>,
    output_file: String,
) {
    let mut file = File::create(&output_file).expect("Failed to create file");

    for k in 0..times.len() - 1 {
        let m = (readings_core[k + 1] - readings_core[k]) / (times[k + 1] - times[k]);
        let b = readings_core[k + 1] - m * times[k + 1];
        writeln!(
            file,
            "{:>6} <= x <= {:>6} ; y = {:>10.4} + {:>10.4} x ; interpolation",
            times[k],
            times[k + 1],
            b,
            m
        )
        .expect("Failed to write to file");
    }
}
