use std::io;

fn main() {
    let n: usize = read_buffer().parse().unwrap();
    let mut a_seq: Vec<usize> = Vec::new();
    for i in 0..=n {
        if i == 0 {
            a_seq.push(1);
        } else {
            a_seq.push(a_seq.iter().map(|&x| f(x)).sum());
        }
    }
    println!("{}",a_seq[n]);
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}

fn f(x: usize) -> usize {
    let mut sum = 0;
    let x_string_vec = x.to_string().chars().collect::<Vec<char>>();
    let x_usize_vec = x_string_vec
        .iter()
        .map(|&t| t.to_string().parse().unwrap())
        .collect::<Vec<usize>>();
    for i in 0..x.to_string().len() {
        sum += x_usize_vec[i];
    }
    sum
}
