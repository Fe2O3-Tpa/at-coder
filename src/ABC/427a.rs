use std::io;

fn main() {
    let input = read_buffer();
    let s = input;
    let mut s_vec: Vec<char> = s.chars().collect();
    s_vec.remove((s.len()+1)/2-1);
    let output: String = s_vec.iter().collect();
    println!("{}",output)
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}