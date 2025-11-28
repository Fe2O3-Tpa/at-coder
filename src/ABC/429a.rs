use std::io;

fn main() {
    let input = read_buffer_vec();
    let n: usize = input[0].parse().unwrap();
    let m: usize = input[1].parse().unwrap();
    
    for i in 1..=n {
        if i <= m {
            println!("OK");
        } else {
            println!("Too Many Requests");
        }
    }
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}
