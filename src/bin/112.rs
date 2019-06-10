//! bouncy numbers
//! increasing number: 134468
//! decreasing number: 66420
//! otherwise bouncy
//!
//! find least number for which proportion of bouncy numbers is exactly 99%

use euler::to_digits;

fn main() {
    let mut n = 100;
    let mut count = 100;
    let mut bouncy_count = 0;

    while (bouncy_count as f64 / count as f64) !=  0.99000 {
        n += 1;
        if is_bouncy(n) {
            bouncy_count += 1;
        }
        count += 1;
        //println!("{}, {}", n, bouncy_count as f32 / count as f32);
        //if n == 21780 { break; }
    }
    println!("{}", n);
}

fn is_bouncy(n: usize) -> bool {
    let digits = to_digits(n);

    let mut direction = Direction::Init;

    for digit_pair in digits.windows(2) {
        let curr_direction: Direction;

        if digit_pair[0] < digit_pair[1] {
            curr_direction = Direction::Increasing;
        } else if digit_pair[0] > digit_pair[1] {
            curr_direction = Direction::Decreasing;
        } else {
            curr_direction = Direction::NoChange;
        }

        if [Direction::Init, Direction::NoChange].contains(&direction) {
            direction = curr_direction.clone();
            continue;
        } else if curr_direction == Direction::NoChange {
            continue;
        } else {
            if direction != curr_direction {
                return true;
            }
        }

        direction = curr_direction.clone();
    }

    false
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Init,
    Increasing,
    Decreasing,
    NoChange,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bouncy() {
        assert_eq!(is_bouncy(134468), false);
        assert_eq!(is_bouncy(66420), false);
        assert_eq!(is_bouncy(155349), true);
        assert_eq!(is_bouncy(540), false);
        assert_eq!(is_bouncy(1586986), true);
    }
}
