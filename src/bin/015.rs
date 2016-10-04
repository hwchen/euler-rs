// lattice paths. If I remember correctly, this one involved
// pascal's triangle?

// nth row of pascal triangle
fn pascal_triangle_row(n: u64) -> Vec<u64> {
    match n {
        0 => vec![],
        _ => {
            let mut pascal_row = vec![1];
            for _ in 1..n {
                let mut current_row = vec![1];
                for win in pascal_row.windows(2) {
                    current_row.push(win[0] + win[1]);
                }
                current_row.push(1);
                pascal_row = current_row;
            }
            pascal_row
        },
    }
}

fn main() {

    let row = 20 + 20 + 1 as u64;
    let middle_index = (row /2) as usize; // don't forget index starts at 0
    println!("{}", middle_index);
    let res = pascal_triangle_row(row)[middle_index];
    println!("Number of paths: {}", res);
//    for i in 1..10 {
//        println!("{:?}", pascal_triangle_row(i));
//    }
}

//1 -> 1
//1 1
//1 2 1 -> 2
//1 3 3 1
//1 4 6 4 1 -> 6
//1 5 10 10 5 1
//1 6 15 20 15 6 1 -> 20
//1 7 21 35 35 21 7 1
//1 8 28 56 70 56 28 8 1

