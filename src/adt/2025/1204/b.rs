use std::io;

fn main() {
    let s: String = read_buffer();
    
    let output = if s == String::from("Hello,World!") {
        String::from("AC")
    } else {
        String::from("WA")
    };

    println!("{}", output)
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}