use std::io;

fn main() {
    read_buffer();
    let s: String = read_buffer();
    let s_vec = s.chars().collect::<Vec<char>>();
    let mut output = String::new();
    for i in s_vec {
        output.push(i);
        output.push(i);
    }

    println!("{}", output)
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}