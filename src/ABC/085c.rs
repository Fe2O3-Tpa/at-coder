use std::io;

fn main() {
    let input = read_buffer_vec();

    let n: isize = input[0].parse().unwrap();
    let Y: isize = input[1].parse().unwrap();
    let mut output: Vec<isize> = Vec::new();

    'outerest: for x in 0..=n {
        for y in 0..=(n - x) {
            let z = n - (x + y);
            if 10000 * x + y * 5000 + z * 1000 == Y {
                output.push(x);
                output.push(y);
                output.push(z);
                break 'outerest;
            }
        }
    }

    if output == Vec::new() {
        output.push(-1);
        output.push(-1);
        output.push(-1);
    }

    println!(
        "{}",
        output
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}
