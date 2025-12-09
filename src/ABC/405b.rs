use std::io;

fn main() {
    let input1: Vec<usize> = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();
    let n = input1[0];
    let m = input1[1];

    let mut a_sequence: Vec<usize> = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();
    let mut loop_var: usize = 0;
    'main_for: for _ in 0..n{
        for i in 1..=m {
            if !a_sequence.contains(&i) {
                break 'main_for;
            }
        }
        a_sequence.pop();
        loop_var += 1;
    }
    println!("{}",loop_var.to_string());
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}