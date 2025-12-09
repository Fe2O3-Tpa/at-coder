use std::{fs::read, io};

fn main() {
    let p = read_buffer();
    let l: usize = read_buffer().parse().unwrap();
    match p.chars().count() >= l {
        true => println!("Yes"),
        false => println!("No")
    }
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}