use std::io;

fn main() {
    let input = read_buffer_vec()
        .iter()
        .map(|t| t.parse().unwrap())
        .collect::<Vec<f32>>();
    let w = input[0];
    let b = input[1];
    let mut output = 1;

    for i in 0..10000001 {
        let i_f = i as f32;
        if ((1000.0 * w) / b) < i_f {
            output = i;
            break;
        }
    }

    println!("{}", output.to_string());
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}
