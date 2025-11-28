use std::io;

fn main() {
    let input = read_buffer_vec();
    let n: usize = input[0].parse().unwrap();
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}