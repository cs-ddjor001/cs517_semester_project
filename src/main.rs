use std::fs::File;
use std::path::Path;

use cs517_semester_project::cubic_spline_interpolation;
use cs517_semester_project::least_squares_approximation;
use cs517_semester_project::piece_wise_linear_interpolation;
use cs517_semester_project::temperature_parser;

fn main() {
    let file_path = std::env::args().nth(1).expect("Usage: program <file_path>");

    let temperature_data = temperature_parser::read_temperature_file(&file_path)
        .expect("Failed to read temperature file");

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

    let output_file_00 = format!("{}-core-00.txt", base_file_name);
    let output_file_01 = format!("{}-core-01.txt", base_file_name);
    let output_file_02 = format!("{}-core-02.txt", base_file_name);
    let output_file_03 = format!("{}-core-03.txt", base_file_name);

    let file_00 = File::create(&output_file_00).expect("Failed to create file");
    let file_01 = File::create(&output_file_01).expect("Failed to create file");
    let file_02 = File::create(&output_file_02).expect("Failed to create file");
    let file_03 = File::create(&output_file_03).expect("Failed to create file");

    piece_wise_linear_interpolation::piece_wise_linear_interpolation(
        &times,
        &readings_core_0,
        &file_00,
    );

    least_squares_approximation::least_squares_approximation(&times, &readings_core_0, &file_00);

    //cubic_spline_interpolation::cubic_spline_interpolation(&times, &readings_core_0, &file_00);

    piece_wise_linear_interpolation::piece_wise_linear_interpolation(
        &times,
        &readings_core_1,
        &file_01,
    );

    least_squares_approximation::least_squares_approximation(&times, &readings_core_1, &file_01);

    //cubic_spline_interpolation::cubic_spline_interpolation(&times, &readings_core_1, &file_01);

    piece_wise_linear_interpolation::piece_wise_linear_interpolation(
        &times,
        &readings_core_2,
        &file_02,
    );

    least_squares_approximation::least_squares_approximation(&times, &readings_core_2, &file_02);

    //cubic_spline_interpolation::cubic_spline_interpolation(&times, &readings_core_2, &file_02);

    piece_wise_linear_interpolation::piece_wise_linear_interpolation(
        &times,
        &readings_core_3,
        &file_03,
    );

    least_squares_approximation::least_squares_approximation(&times, &readings_core_3, &file_03);

    //cubic_spline_interpolation::cubic_spline_interpolation(&times, &readings_core_3, &file_03);
}
