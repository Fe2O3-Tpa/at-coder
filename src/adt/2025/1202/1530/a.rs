use std::io;

fn main() {
    let input1 = read_buffer_vec()
        .iter()
        .map(|t| t.parse().unwrap())
        .collect::<Vec<usize>>();

    let a = input1[1];
    let b = input1[2];

    let input2 = read_buffer_vec()
        .iter()
        .map(|t| t.parse().unwrap())
        .collect::<Vec<usize>>();

    for i in 0..input2.len() {
        if a+b == input2[i] {
            println!("{}", (i+1).to_string());
            break;
        }
    }
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}
