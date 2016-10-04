fn main() {
    let sum = (1..1000).filter(|i| {
        i % 3 == 0 ||
        i % 5 == 0
    }).fold(0, |sum, x| {
        sum + x
    });

    println!("Sum of multiples of 3 and 5 below 10: {}", sum);
}
