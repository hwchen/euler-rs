fn multiply_by_two(n: &mut Vec<u64>) {
    let mut carry = 0;

    for i in 0..n.len() {
        let current_digit = n[i];
        let mut update = current_digit * 2 + carry;

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
        // multiplying 2 * 9
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
    // the ugly way: I can literally multiply digits 1000 times.
    // To make it simple, the bignum representation is going to be
    // base 10 per slot.
    // also, the bignum representation is "backwards", with tail being
    // the higher 10s place.

    let mut num = vec![2];
    //println!("Initial num: {:?}", num);

    for _ in 2..1001 {
        ////println!("\nexponent {:?}", i);
        ////println!("multiply {:?} by 2:", num);

        multiply_by_two(&mut num);
        ////println!("{:?}", num);

    }

    //println!("comparison of 2^20: {}", 2u32.pow(20));
    //println!("Final number: {:?}", num);
    let digits_sum = num.into_iter().fold(0, |sum, i| sum + i);
    println!("Final number digit sum: {:?}", digits_sum);

}
