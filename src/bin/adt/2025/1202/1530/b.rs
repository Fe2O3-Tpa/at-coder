use std::io;

fn main() {
    let input1 = read_buffer_vec()
        .iter()
        .map(|t| t.parse().unwrap())
        .collect::<Vec<usize>>();

    let x = input1[0];
    let y = input1[1];

    if y <= 2 {
        println!("{}-",x.to_string());
    } else if y<=6 {
        println!("{}",x.to_string());
    } else if y<=9 {
        println!("{}+",x.to_string());
    }
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(".").map(|s| s.to_string()).collect()
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}
