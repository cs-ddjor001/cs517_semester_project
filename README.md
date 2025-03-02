# CPU Temperature Analysis #

# Overview #

This project processes CPU temperature readings from text files, performing:
- Piecewise Linear Interpolation

- Least Squares Approximation

- (Optional) Cubic Spline Interpolation

# Input format #

Input files contain temperature readings from four CPU cores, recorded every 30 seconds. Example:
```
61.0 63.0 50.0 58.0
80.0 81.0 68.0 77.0
```
Each row represents a timestamp, and each column corresponds to a core.

# Output format #

For each core, the program generates a text file with interpolated and least-squares approximated values:
```
0 <= x < 30 ; y = 61.0000 + 0.6333 x ; interpolation
0 <= x <= 120 ; y = 67.4000 + 0.0567 x ; least-squares
```
# Usage #

Run the program with an input filename from the data directory:
```
cargo run -- .\data\sensors-2019.01.26-no-labels.txt
```
# Architecture #

The project is modularized into:

- Input Handling
- Data Pre-processing
- Interpolation Module
- Least Squares Approximation Module
