use std::io;

fn main() {
    let input = read_buffer_vec();
    let s: usize = input[0].parse().unwrap();
    let a: usize = input[1].parse().unwrap();
    let b: usize = input[2].parse().unwrap();
    let x: usize = input[3].parse().unwrap();

    let mut output: usize = 0;
    for i in 0..x {
        if i%(a+b) <= a-1 {
            output += s;
        }
    }
    println!("{}", output.to_string())
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}