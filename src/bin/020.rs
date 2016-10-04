
// modified from euler 016
fn multiply_by_x(n: &mut Vec<u64>, x: u64) {
    let mut carry = 0;

    for i in 0..n.len() {
        let current_digit = n[i];
        let mut update = current_digit * x + carry;

        // process the new number for carry-over
        if update > 9 {
            carry = update / 10;
            update = update % 10;
        } else {
            carry = 0;
        }

        // update
        n[i] = update;

        // deal with carry-over for last operation: put
        // all digits on.
        // ?? What happens if the carry keeps growing until
        // larger than u64?
        // Oh, in this case it's fine because it's maximum
        // multiplying by 9 + carry, max carry can only be 8 so can
        // never have 2 digit carry.
        if i == n.len()-1 && carry != 0 {
            loop {
                if carry == 0 { break; }

                n.push(carry % 10);
                carry = carry / 10;
            }
        }
    }
}

fn main() {
    let mut n = vec![0,0,1];

    for i in 2..1_00 {
        multiply_by_x(&mut n, i);
    }

    let sum_digits = n.into_iter().fold(0, |sum, x| sum + x);

    println!("Sum of digits in 100!: {}", sum_digits);
}
