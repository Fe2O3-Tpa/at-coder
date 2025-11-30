use std::io;

fn main() {
    let n: usize = read_buffer().parse().unwrap();
    
    let mut can_kuku = false;
    for h in 1..=9 {
        for i in 1..=9 {
            if h*i == n {
                can_kuku = true;
            }
        }
    }

    match can_kuku {
        true => println!("Yes"),
        false => println!("No")
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
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}

fn to_vec_usize(v: Vec<String>) -> Vec<usize> {
    let output: Vec<usize> = v
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    output
}