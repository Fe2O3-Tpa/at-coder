use std::io;

fn main() {
    let input = read_buffer_vec()
        .iter()
        .map(|t| t.parse().unwrap())
        .collect::<Vec<usize>>();
    let x = input[0];
    let y = input[1];

    let mut a_seq: Vec<usize> = Vec::new();
    for i in 0..10 {
        a_seq.push(match i {
            0 => x,
            1 => y,
            _ => f(a_seq[i - 1] + a_seq[i - 2]),
        });
    }

    println!("{}", a_seq[9].to_string());
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}

fn f(input: usize) -> usize {
    let v: Vec<char> = input.to_string().chars().collect();
    let reverced_string = v.iter().rev().collect::<String>();
    reverced_string.parse::<usize>().unwrap()
}
