use std::io;

fn main() {
    let input = read_buffer_vec();
    let n: usize = input[0].parse().unwrap();
    let m: usize = input[1].parse().unwrap();

    let a_sequence: Vec<usize> = read_buffer_vec().iter().map(|s| s.parse().unwrap()).collect();

    let mut output: bool = false;
    for i in 0..n {
        let mut a_sequence_removed = a_sequence.clone();
        a_sequence_removed.remove(i);
        if a_sequence_removed.iter().sum::<usize>() == m {
            output = output || true;
            break;
        }
    }
    match output {
        true => println!("Yes"),
        false => println!("No")
    }
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}