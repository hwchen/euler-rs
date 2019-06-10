//! bouncy numbers
//! increasing number: 134468
//! decreasing number: 66420
//! otherwise bouncy
//!
//! find least number for which proportion of bouncy numbers is exactly 99%

use euler::to_digits;

fn main() {

}

fn is_bouncy(n: usize) -> bool {
    let digits = to_digits(n);

    let mut direction = Direction::Init;

    for digit_pair in digits.windows(2) {
        let curr_direction: Direction;

        if digit_pair[0] < digit_pair[1] {
            curr_direction = Direction::Increasing;
        } else if digit_pair[0] < digit_pair[1] {
            curr_direction = Direction::Decreasing;
        } else if digit_pair[0] == digit_pair[1] {
            curr_direction = Direction::NoChange;
        }

        if direction == Init || curr_direction == Direction::NoChange {
            continue;
        } else {
            if direction != curr_direction {
                return true;
            }
        }

        direction = curr_direction;
    }

    false
}

#[derive(Debug, PartialEq)]
enum Direction {
    Init,
    Increasing,
    Decreasing,
    NoChange,
}
