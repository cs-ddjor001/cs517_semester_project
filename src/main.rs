//! # Temperature Data Processing
//!
//! This module reads temperature data from a file, parses it, and performs piecewise linear interpolation.
//! It writes the interpolated results to separate output files for each core.

use regex::Regex;
use std::cell::LazyCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use thiserror::Error;

/// Represents a single line of temperature data.
#[derive(Debug)]
pub struct TemperatureLine {
    pub time_step: u64,
    pub readings: Vec<f64>,
}

/// Errors that may occur while parsing the temperature data.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

//------------------------------------------------------------------------------
const TIME_STEP_SIZE: u64 = 30;

const LINE_DELIM_RE: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"[^0-9]*\s+|[^0-9]*$").unwrap());

/// Reads temperature data from a file and returns a vector of `TemperatureLine`.
///
/// # Arguments
/// * `filename` - The path to the temperature data file.
///
/// # Returns
/// A `Result` containing a vector of `TemperatureLine` or a `ParseError`.
pub fn read_temperature_file(filename: &str) -> Result<Vec<TemperatureLine>, ParseError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    read_temperatures(reader)
}

/// Reads temperature data from a buffered reader and returns a vector of `TemperatureLine`.
///
/// # Arguments
/// * `reader` - A buffered reader implementing `BufRead`.
///
/// # Returns
/// A `Result` containing a vector of `TemperatureLine` or a `ParseError`.
pub fn read_temperatures<R>(reader: R) -> Result<Vec<TemperatureLine>, ParseError>
where
    R: BufRead,
{
    let mut readings: Vec<TemperatureLine> = Vec::new();

    for (idx, wrapped_line) in reader.lines().enumerate() {
        let line = wrapped_line?;
        let time = (idx as u64) * TIME_STEP_SIZE;

        let core_temps: Vec<f64> = LINE_DELIM_RE
            .split(line.trim())
            .into_iter()
            .filter_map(|s| s.parse().ok())
            .collect();

        let new_reading = TemperatureLine {
            time_step: time,
            readings: core_temps,
        };

        readings.push(new_reading);
    }

    Ok(readings)
}

fn main() {
    let file_path = std::env::args().nth(1).expect("Usage: program <file_path>");

    let temperature_data =
        read_temperature_file(&file_path).expect("Failed to read temperature file");

    let mut times: Vec<f64> = vec![];
    let mut readings_core_0: Vec<f64> = vec![];
    let mut readings_core_1: Vec<f64> = vec![];
    let mut readings_core_2: Vec<f64> = vec![];
    let mut readings_core_3: Vec<f64> = vec![];

    let path = Path::new(&file_path);
    let base_file_name = path.file_stem().unwrap().to_str().unwrap();

    for temp in temperature_data {
        times.push(temp.time_step as f64);

        if temp.readings.len() >= 4 {
            readings_core_0.push(temp.readings[0]);
            readings_core_1.push(temp.readings[1]);
            readings_core_2.push(temp.readings[2]);
            readings_core_3.push(temp.readings[3]);
        } else {
            eprintln!("Warning: Incomplete temperature data on some lines.");
        }
    }

    piece_wise_linear_interpolation(
        &base_file_name,
        times,
        readings_core_0,
        readings_core_1,
        readings_core_2,
        readings_core_3,
    );
}

/// Performs piecewise linear interpolation on the temperature data and writes results to files.
///
/// # Arguments
/// * `base_file_name` - The base name for the output files.
/// * `times` - A vector of time steps.
/// * `readings_core_0` - A vector of vectors containing temeperature readings for core 0
/// * `readings_core_1` - A vector of vectors containing temeperature readings for core 1
/// * `readings_core_2` - A vector of vectors containing temeperature readings for core 2
/// * `readings_core_3` - A vector of vectors containing temeperature readings for core 3.
fn piece_wise_linear_interpolation(
    base_file_name: &str,
    times: Vec<f64>,
    readings_core_0: Vec<f64>,
    readings_core_1: Vec<f64>,
    readings_core_2: Vec<f64>,
    readings_core_3: Vec<f64>,
) {
    let output_file_00 = format!("{}-core-00.txt", base_file_name);
    let output_file_01 = format!("{}-core-01.txt", base_file_name);
    let output_file_02 = format!("{}-core-02.txt", base_file_name);
    let output_file_03 = format!("{}-core-03.txt", base_file_name);

    let mut file_00 = File::create(&output_file_00).expect("Failed to create file");
    let mut file_01 = File::create(&output_file_01).expect("Failed to create file");
    let mut file_02 = File::create(&output_file_02).expect("Failed to create file");
    let mut file_03 = File::create(&output_file_03).expect("Failed to create file");

    for k in 0..times.len() - 1 {
        let m = (readings_core_0[k + 1] - readings_core_0[k]) / (times[k + 1] - times[k]);
        let _b = readings_core_0[k + 1] - m * times[k + 1];
        writeln!(
            file_00,
            "{:>6} <= x <= {:>6} ; y = {:>10.4} + {:>10.4} x ; interpolation",
            times[k],
            times[k + 1],
            readings_core_0[k],
            m
        )
        .expect("Failed to write to file");
    }

    for k in 0..times.len() - 1 {
        let m = (readings_core_1[k + 1] - readings_core_1[k]) / (times[k + 1] - times[k]);
        let _b = readings_core_1[k + 1] - m * times[k + 1];
        writeln!(
            file_01,
            "{:>6} <= x <= {:>6} ; y = {:>10.4} + {:>10.4} x ; interpolation",
            times[k],
            times[k + 1],
            readings_core_1[k],
            m
        )
        .expect("Failed to write to file");
    }

    for k in 0..times.len() - 1 {
        let m = (readings_core_2[k + 1] - readings_core_2[k]) / (times[k + 1] - times[k]);
        let _b = readings_core_2[k + 1] - m * times[k + 1];
        writeln!(
            file_02,
            "{:>6} <= x <= {:>6} ; y = {:>10.4} + {:>10.4} x ; interpolation",
            times[k],
            times[k + 1],
            readings_core_2[k],
            m
        )
        .expect("Failed to write to file");
    }

    for k in 0..times.len() - 1 {
        let m = (readings_core_3[k + 1] - readings_core_3[k]) / (times[k + 1] - times[k]);
        let _b = readings_core_3[k + 1] - m * times[k + 1];
        writeln!(
            file_03,
            "{:>6} <= x <= {:>6} ; y = {:>10.4} + {:>10.4} x ; interpolation",
            times[k],
            times[k + 1],
            readings_core_3[k],
            m
        )
        .expect("Failed to write to file");
    }
}
