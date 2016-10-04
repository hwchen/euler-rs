// pandigital multiplican, multiplier, product

// 1-9 pandigital (nine total digits)
// product must be at least 4 digits, otherwise if 3, than
//   one of the others are 3 digits,
//   which will definitely be more than 3 digits.
// product must be fewer than 5 digits; 98 * 76 is not enough to reach 5 digits
//
// mult/mult combo can be 1/4, 2/3

// is it better to generate all permutations first?
//
// rust is .06, hask is 0.40.
// hask a little less efficient because it checks a wider range of multipliers.

fn main() {
    let mut res = vec![];

    // 1/4 combo
    for a in 1..10 {
        for b in 1234..9877 {
            let product = a * b;
            let mut digits: Vec<_> = format!("{}{}{}", a, b, product)
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();

            // sort and compare?
            // or look for duplicates? which is faster?
            // TODO probabky direct compare
            if digits.len() == 9 {
                digits.sort();
                digits.dedup();
                if digits.len() == 9 && digits[0] != 0 {
                    res.push(product);
                }
            }
        }
    }

    // 2/3 combo
    for a in 12..99 {
        for b in 123..988 {
            let product = a * b;
            let mut digits: Vec<_> = format!("{}{}{}", a, b, product)
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();

            // sort and compare?
            // or look for duplicates? which is faster?
            // TODO probabky direct compare
            if digits.len() == 9 {
                digits.sort();
                digits.dedup();
                if digits.len() == 9 && digits[0] != 0 {
                    res.push(product);
                }
            }
        }
    }

    res.sort();
    res.dedup();

    println!("pandigitals products: {:?}", res);
    println!("pandigitals sum: {}", res.iter().sum::<usize>());
}
