// making pounds from pence
//
// remember, that the 1 pence are just fillers,
// so don't have to consider them in the permutations.
//
// 2p, 5p, 10p, 50p, 1pound, 2pound.
//
// So, beforehand know these:
//
// - all 1p
// - just 2p
//
// All other calculations just have to fall under 200p
//
// rust is 0.08, hask is 0.16

fn main() {
    let twop = 2;
    let fivep = 5;
    let tenp = 10;
    let twentyp = 20;
    let fiftyp = 50;
    let onelb = 100;

    let mut count = 0;

    for a in 0..101 { //2p
        for b in 0..41 { //5p
            for c in 0..21 { //10p
                for d in 0..11 { //20p
                    for e in 0..5 { //50p
                        for f in 0..3 { //1lb
                            if  a * twop +
                                b * fivep +
                                c * tenp +
                                d * twentyp +
                                e * fiftyp +
                                f * onelb <= 200 {

                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    // count is only +1, because all zeros is equal to all 1p and already
    // counted
    println!("total permutations (coins): {}", count + 1);
}
