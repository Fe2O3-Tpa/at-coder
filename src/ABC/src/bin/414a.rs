use std::io;

fn main() {
    let input_1 = read_buffer_vec();
    let n: usize = input_1[0].parse().unwrap();
    let l: usize = input_1[1].parse().unwrap();
    let r: usize = input_1[2].parse().unwrap();

    let mut listener_data: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        let listener_push: Vec<usize> = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();
        listener_data.push(listener_push);
    }

    let mut output_usize: usize = 0;
    for i in 0..n {
        if listener_data[i][0] <= l && listener_data[i][1] >= r {
            output_usize += 1;
        }
    }

    println!("{}", output_usize.to_string());
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}