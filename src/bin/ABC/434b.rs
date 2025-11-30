use std::io;

fn main() {
    let input = read_buffer_vec()
        .iter()
        .map(|t| t.parse().unwrap())
        .collect::<Vec<usize>>();
    let n = input[0];
    let m = input[1];

    let mut a: Vec<f64> = Vec::new();
    let mut b: Vec<f64> = Vec::new();
    for _ in 0..n {
        let input_some = read_buffer_vec()
            .iter()
            .map(|t| t.parse().unwrap())
            .collect::<Vec<f64>>();
        a.push(input_some[0]);
        b.push(input_some[1]);
    }

    for h in 0..m{
        let mut sum_vec: Vec<f64> = Vec::new();
        // hである鳥の数
        let mut h_kazu: f64 = 0.0;
        for i in 0..n {
            if a[i] == (h+1) as f64 {
                sum_vec.push(b[i]);
                h_kazu += 1.0;
            }
        }
        let ave: f64 = sum_vec.iter().sum::<f64>() / h_kazu;
        println!("{}", ave.to_string());
    }
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}
