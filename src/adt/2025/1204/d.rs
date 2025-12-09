use std::io;

fn main() {
    let input1: Vec<usize> = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();
    let n = input1[0];
    let m = input1[1];

    let a_seq: Vec<usize> = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();
    let b_seq: Vec<usize> = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();

    let mut used: Vec<bool> = Vec::new();
    for i in 0..n {
        used.push(false);
    }

    let mut can_every_date = true;
    'outer: for date in 0..m {
        let mut can_date = false;
        'inner: for pasta_kind in 0..n {
            if a_seq[pasta_kind] == b_seq[date] && !used[pasta_kind] {
                can_date = true;
                used[pasta_kind] = true;
                break 'inner;
            }
        }
        if !can_date {
            can_every_date = false;
            break 'outer;
        }
    }

    let output = if can_every_date {
        "Yes"
    } else {
        "No"
    };
    println!("{}", output);
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
    .read_line(&mut buffer)
    .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}