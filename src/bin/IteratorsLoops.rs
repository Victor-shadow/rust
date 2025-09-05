fn main() {
    // Initialize buffer with at least 24 elements to allow indexing from i = 12 onward
    let mut buffer: Vec<i32> = vec![0; 24];
    // Fill the first 12 elements with sample data
    for i in 0..12 {
        buffer[i] = i as i32;
    }

    // Coefficients for prediction (example values)
    let coefficients: [i64; 12] = [1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6];
    let qlp_shift: i16 = 4;

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>() >> qlp_shift;

        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }

    println!("{:?}", buffer);
}
//To calculate the value of prediction, the code iterates through each of the 12 values in coefficients and uses the zip method
//to pair the coefficient values with the previous 12 values in buffer. then for each pair, multiply the values together, sum all the results
//and shifts the bits in the sum qlp_shift bits to the right
//All the coefficients get stored in Registers , which means accessing the values is fast
//There are no bound checks on array access at runtime