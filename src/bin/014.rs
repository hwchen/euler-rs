// rust is .19s, haskell is 20s

//input starting n, output length of Collatz sequence
fn collatz_len(x: u64) -> u64 {
    let mut count = 0;
    let mut n = x;

    // input n in sequence, output next n in sequence
    loop {
        count += 1;
        if n == 1 {
            break;
        } else if n % 2 == 0 {
            n = n/2;
        } else {
            n = 3 * n + 1;
        }
    };

    count
}

fn main() {
    let mut longest_seq = 0;
    let mut longest_seq_n = 0;
    for i in 2..1000000 {
        let seq_length = collatz_len(i);
        if seq_length > longest_seq {
            longest_seq = seq_length;
            longest_seq_n = i;
        }
    }

    println!("Longest Collatz seq starting under 1000000: {}, len {}", longest_seq_n, longest_seq);
}
