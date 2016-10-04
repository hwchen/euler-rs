
fn main() {
    let sum_of_squares = (1..101)
        .map(|i| i * i)
        .fold(0, |sum, x| sum + x);

    let sum = (1..101)
        .fold(0, |sum, x| sum + x);

    let square_of_sums = sum * sum;

    println!("{}", square_of_sums);

    println!("diff of square of sums and sum of squares for 1-1000: {}",
             (square_of_sums as i64- sum_of_squares as i64).abs());
}
