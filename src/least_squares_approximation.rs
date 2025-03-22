use std::fs::File;
use std::io::Write;

pub fn least_squares_approximation(times: &Vec<f64>, readings_core: Vec<f64>, output_file: String) {
    let mut file = File::create(&output_file).expect("Failed to create file");

    let k = times.len() - 1;

    let b = 0.0;
    let m = 0.0;

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

/*let mut sum_xi = 0.0;
let mut sum_fxi = 0.0;
let mut sum_xi_squared = 0.0;
let mut sum_xi_fi = 0.0;

let k = times.len();

for i in 0..k - 1 {
    sum_xi += times[i];
    sum_fxi += readings_core[i];
    sum_xi_fi += times[i] * readings_core[i];
    sum_xi_squared += times[i] * times[i];
}

let sum_squared_xi = sum_xi_squared * sum_xi_squared;

let m =
    (k as f64 * sum_xi_fi - sum_xi * sum_fxi) / (k as f64 * sum_xi_squared - sum_squared_xi);

let b = (sum_fxi - m * sum_xi) / (k as f64);*/
