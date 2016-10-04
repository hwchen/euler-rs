// pythagorean triplet a^2 + b^2 = c^2
// there exists one pythagorean triplet for which a + b + c = 1000

// using while loops to break early, cuts down time a ton from
// using a for loop it was maybe 1.5 seconds with for loop, only
// .09 seconds with while loops
//
// But there were a ton of spots for making bugs!
fn main() {

    let mut soln = 0;
    let mut a = 1;
    let mut b = 1;
    let mut c = 1;

    'outer: loop {
        loop {
            loop {
                if ((a*a + b*b) == c*c) && a + b + c == 1000 {
                    println!("pythagorean triplet: {} {} {}", a, b, c);
                    soln = a * b * c;
                    break 'outer;
                }

                c += 1;
                if c == 1000 {
                    c = 1;
                    break;
                }
            }

            b += 1;
            if b == 1000 {
                b = 1;
                break;
            }
        }

        a += 1;
        if a == 1000 {break;}
    }
    println!("pythag triplet product: {}", soln);
}
