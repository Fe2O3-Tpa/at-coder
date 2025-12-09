use std::io;
use std::cmp::max;

fn main() {
    let input_1: Vec<String> = read_buffer();
    let mut h_string = String::new();
    let mut b_string = String::new();

    if let Some(z) = input_1.get(0) {
        h_string = z.to_string();
    }
    if let Some(z) = input_1.get(1) {
        b_string = z.to_string();
    }

    let h: i32 = h_string.parse().unwrap();
    let b: i32 = b_string.parse().unwrap();
    let n = max(h-b, 0);
    println!("{}",n.to_string())
}



fn read_buffer() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split_whitespace().map(|s| s.to_string()).collect()
}