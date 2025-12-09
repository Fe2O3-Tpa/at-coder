use std::{cmp::max, io};

fn main() {
    let s = input();
    let mut max_acgt_len: usize = 0;

    for h in 0..s.len() {
        for i in (0..s.len() - h).map(|t| s.len()-t) {
            if s[h..i]
                .chars()
                .collect::<Vec<_>>()
                .iter()
                .all(|c: &char| c == &'A' || c == &'C' || c == &'G' || c == &'T')
            {
                max_acgt_len = max(max_acgt_len, s[h..i].len());
            }
        }
    }

    println!("{}", max_acgt_len.to_string());
}

fn input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}
