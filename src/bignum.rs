// TODO
// - div, sub, ord
// - support neg numbers?
// - clean up and refactor.
// - use new algorithms?
// - I need to get through this thoroughly so that I don't keep coming back
// - Consider better implementation details, support for more ergonomic features

use std::char::from_digit;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BigNum(Vec<usize>);
// methods:
//
// fromStr
// add two bignums together. (implement add trait?)
// digits length
// factorial
//
// TODO oops, haven't beeng checking for empty vecs.

impl BigNum {
    pub fn new(i: Vec<usize>) -> Self {
        BigNum(i)
    }

    pub fn digit_len(&self) -> usize {
        self.0.len()
    }

    pub fn pow(self, power: usize) -> Self {
        let mut res = BigNum::new(vec![1]);

        if power == 0 {
            return res;
        } else {
            for _ in 0..power {
                res = res * self.clone();
            }
        }

        res
    }

    pub fn fac(self) -> Self {
        let mut res = BigNum::new(vec![1]);

        if let Some(&first) = self.0.get(0) {
            // currently not checking for None
            if first == 0 {
                return res;
            }
        }

        let mut counter = BigNum::new(vec![1]);

        while counter != self {
            println!("counter: {:?}, res: {:?}", counter, res);
            // RHS must be moved?
            res = res * counter.clone();

            counter = counter + BigNum::new(vec![1]);
        }

        res * counter
    }
}

impl FromStr for BigNum {
    type Err = ();

    fn from_str(s: &str) -> Result<BigNum, ()> {
        // Lazy for now, just panic on bad parses.

        let res = s.chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();

        Ok(BigNum(res))
    }
}

impl Add for BigNum {
    type Output = BigNum;

    fn add(self, rhs: BigNum) -> BigNum {
        BigNum::new(add_vecs(self.0, rhs.0))
    }
}

impl Mul for BigNum {
    type Output = BigNum;

    fn mul(self, rhs: BigNum) -> BigNum {
        // I'm doing this very simply, top multiply bottom.
        // top and bottom are both immutable here (unlike
        // in addition, there's multiple passes). Can optimize later
        //
        // In the end, will have to generate new clones and 
        //
        // cleanup len calls later
        let mut res = vec![0];
        let mut carry = 0;

        // set vars for mutable result, the number to add it to.
        // result is the longer number or equal
        let (top, bottom) = if self.0.len() >= rhs.0.len() {
            (self.0, rhs.0)
        } else {
            (rhs.0, self.0)
        };

        for k in 0..bottom.len() {
            let bottom_current_digit = bottom[k];

            // pad another zero for each place over
            let pad = k;

            // vec same length as top for intermediate product
            // carry over can only add one more place max?
            let mut intermediate_product = vec![0; top.len() + pad];

            // each time through this loop is multiplying times
            // one digit on bottom. At end of loop, add to res
            for i in 0..top.len() {
                let top_current_digit = top[i];

                // multiply "top" digit by "bottom" digit
                let mut update = top_current_digit * bottom_current_digit + carry;

                // process the new number for carry-over
                if update > 9 {
                    carry = update / 10;
                    update = update % 10;
                } else {
                    carry = 0;
                }

                // update
                intermediate_product[i + pad] = update;

                // deal with carry-over for last operation: put
                // all digits on.
                // ?? What happens if the carry keeps growing until
                // larger than u64?
                if i + pad == intermediate_product.len() - 1 && carry != 0 {
                    loop {
                        if carry == 0 { break; }

                        intermediate_product.push(carry % 10);
                        carry = carry / 10;
                    }
                }
            }

            //zip together result vec and intermediate product vec and add
            //but the zip stops at the longer, not shorter vec. so need to
            //do manually in loop (intermediate product should always be
            //longer

            // just use and allocate another bignum for now.
            //for i in 0..intermediate_product.len() {
            //    if res.get(i).is_none() {
            //        res.push(0);
            //    }
            //    res[i] += intermediate_product[i];
            //}

            // Why is creating a bignum faster than just using vecs?
            // Is here something about the memory layout?
            res = (BigNum::new(res) + BigNum::new(intermediate_product)).0;
            //res = add_vecs(res, intermediate_product);
        }

        BigNum::new(res)
    }
}

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = self.0.iter()
            .rev()
            .map(|&digit| from_digit(digit as u32, 10).expect("not a digit in display"))
            .collect::<String>();
        write!(f, "{}", out)
    }
}

fn add_vecs(left: Vec<usize>, right: Vec<usize>) -> Vec<usize> {
    let mut carry = 0;

    // set vars for mutable result, the number to add it to.
    // result is the longer number or equal
    let (mut res, adder) = if left.len() >= right.len() {
        (left.clone(), right)
    } else {
        (right.clone(), left)
    };

    for i in 0..res.len() {
        let res_current_digit = res[i];

        // add only for the length of the adder
        // (this way don't have to pad!)
        let adder_current_digit = if i < adder.len() {
            adder[i]
        } else {
            0
        };

        let mut update = res_current_digit + adder_current_digit + carry;

        // process the new number for carry-over
        if update > 9 {
            carry = update / 10;
            update = update % 10;
        } else {
            carry = 0;
        }

        // update
        res[i] = update;

        // deal with carry-over for last operation: put
        // all digits on.
        // ?? What happens if the carry keeps growing until
        // larger than u64?
        if i == res.len()-1 && carry != 0 {
            loop {
                if carry == 0 { break; }

                res.push(carry % 10);
                carry = carry / 10;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bignum_test() {
        let one = "1".parse::<BigNum>().unwrap();
        let two = BigNum::new(vec![2]);
        let three = BigNum::new(vec![3]);
        let four = BigNum::new(vec![4]);
        let nine = BigNum::new(vec![9]);
        let nineteen = BigNum::new(vec![9,1]);
        assert_eq!(one + two, BigNum::new(vec![3]));
        assert_eq!(three + nine, BigNum::new(vec![2,1]));
        assert_eq!(nineteen + four, BigNum::new(vec![3,2]));

        // making sure that having longer one second also works
        let four = BigNum::new(vec![4]);
        let nineteen = BigNum::new(vec![9,1]);
        assert_eq!(four + nineteen, BigNum::new(vec![3,2]));

        let oneninefive = BigNum::new(vec![5,9,1]);
        println!("{}", oneninefive);
        //assert!(false);
    }

    #[test]
    fn bignum_mult_test() {
        let one = "1".parse::<BigNum>().unwrap();
        let two = BigNum::new(vec![2]);
        let three = BigNum::new(vec![3]);
        let four = BigNum::new(vec![4]);
        let nine = BigNum::new(vec![9]);
        let nineteen = BigNum::new(vec![9,1]);
        assert_eq!(one * two, BigNum::new(vec![2]));
        assert_eq!(three * nine, BigNum::new(vec![7,2]));
        assert_eq!(nineteen * four, BigNum::new(vec![6,7]));

        // making sure that having longer one second also works
        let four = BigNum::new(vec![4]);
        let nineteen = BigNum::new(vec![9,1]);
        assert_eq!(four * nineteen, BigNum::new(vec![6,7]));

        // slightly longer number
        let twentyfour = BigNum::new(vec![4,2]);
        let onehundredfiftytwo = BigNum::new(vec![2,5,1]);
        assert_eq!(twentyfour * onehundredfiftytwo, BigNum::new(vec![8,4,6,3]));

        // this one tests for carryover
        let ninetynine = BigNum::new(vec![9,9]);
        let ninetynine1 = BigNum::new(vec![9,9]);
        assert_eq!(ninetynine * ninetynine1, BigNum::new(vec![1,0,8,9]));

        let nineteen_1 = BigNum::new(vec![9,1]);
        let nineteen_2 = BigNum::new(vec![9,1]);
        assert_eq!(nineteen_1 * nineteen_2, BigNum::new(vec![1,6,3]));
        //assert!(false);
    }

    #[test]
    fn bignum_pow_test() {
        let one = "1".parse::<BigNum>().unwrap();
        let two = "2".parse::<BigNum>().unwrap();
        let three = BigNum::new(vec![3]);
        let nineteen = BigNum::new(vec![9,1]);
        assert_eq!(one.pow(2) , BigNum::new(vec![1]));
        assert_eq!(two.pow(2) , BigNum::new(vec![4]));
        assert_eq!(three.pow(9), BigNum::new(vec![3,8,6,9,1]));
        assert_eq!(nineteen.pow(4), BigNum::new(vec![1,2,3,0,3,1]));

        // making sure that having longer one second also works
        let four = BigNum::new(vec![4]);
        assert_eq!(four.pow(19), BigNum::new(vec![4,4,9,6,0,9,7,7,8,4,7,2]));

        let ten = BigNum::new(vec![0,1]);
        assert_eq!(ten.pow(10), BigNum::new(vec![0,0,0,0,0,0,0,0,0,0,1]));

        let nine = BigNum::new(vec![9]);
        assert_eq!(nine.pow(9), BigNum::new(vec![9,8,4,0,2,4,7,8,3]));

        // this one tests for carryover
        let ninetynine = BigNum::new(vec![9,9]);
        assert_eq!(ninetynine.pow(2), BigNum::new(vec![1,0,8,9]));

        let ninetynine = BigNum::new(vec![9,9]);
        assert_eq!(ninetynine.pow(99), BigNum::new(vec![9,9,8,9,9,4,0,0,2,9,5,1,9,9,9,9,7,7,9,9,9,4,8,3,1,0,3,9,5,6,7,6,6,0,6,0,1,7,0,8,1,4,4,1,0,4,4,6,3,9,1,0,7,5,1,2,6,1,5,4,9,5,3,4,9,4,9,6,7,6,9,9,9,2,4,9,1,1,0,4,5,6,2,3,8,4,5,9,8,8,8,7,9,0,4,9,7,2,7,2,3,2,8,2,1,0,1,0,0,5,6,0,1,4,1,2,4,3,2,5,7,7,2,5,5,4,0,7,5,2,5,5,3,2,4,2,7,9,9,5,2,0,3,4,2,0,1,1,4,7,1,8,2,4,6,7,8,6,6,5,9,5,0,4,4,5,0,8,8,2,6,5,0,9,7,8,1,7,5,6,2,7,7,6,2,7,9,4,6,7,3,6,9,2,7,9,6,3]));

        //assert!(false);
    }

    #[test]
    fn bignum_fac_test() {
        let zero = "0".parse::<BigNum>().unwrap();
        let one = "1".parse::<BigNum>().unwrap();
        let two = "2".parse::<BigNum>().unwrap();
        let six = BigNum::new(vec![6]);
        let thirteen = BigNum::new(vec![3,1]);

        assert_eq!(zero.fac() , BigNum::new(vec![1]));
        assert_eq!(one.fac() , BigNum::new(vec![1]));
        assert_eq!(two.fac() , BigNum::new(vec![2]));
        assert_eq!(six.fac(), BigNum::new(vec![0,2,7]));
        assert_eq!(thirteen.fac(), BigNum::new(vec![0,0,8,0,2,0,7,2,2,6]));

    }
}
