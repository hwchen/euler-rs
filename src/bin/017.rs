fn main() {
    let one_through_nine = [3, 3, 5, 4, 4, 3, 5, 5, 4].into_iter().fold(0, |sum, i| sum + i);
    let eleven_through_nineteen = [6, 6, 8, 8, 7, 7, 9, 8, 8].into_iter().fold(0, |sum, i| sum+i);
    let ten = 3;
    let tens_except_ten = [6, 6, 5, 5, 5, 7, 6, 6].into_iter().fold(0, |sum, i| sum + i);
    let hundred = 7;
    let one_thousand = 11;
    let and = 3;

    let sum_letters =
        one_through_nine * 90 + // for ten hundreds, each hundred uses 9 times
        eleven_through_nineteen * 10 + // for ten hundreds
        ten *10 +
        tens_except_ten * 100 + // for ten hundreds, each hundred uses tens 9 times
        hundred * (1000 - 99 - 1) +
        one_through_nine * 100 + // for the times they're used with hundreds
        and * (1000 - 99 - 9 - 1) +
        one_thousand;

    println!("Sum of letters of written nums from 1 to 1000: {}", sum_letters);

}

// one two three four five six seven eight nine
// eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen
// ten twenty thirty forty fifty sixty seventy eighty ninety
// hundred

