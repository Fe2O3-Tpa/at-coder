use std::io;

fn main() {
    read_buffer();
    let s= read_buffer();
    
    let output = s.matches("ABC").count();
    println!("{}", output.to_string());
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}