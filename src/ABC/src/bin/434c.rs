use std::io;

fn main() {
    let t: usize = read_buffer().parse().unwrap();
    
    // N,Hは0行目。
    let mut case_vec: Vec<Vec<usize>> = Vec::new();
    for h in 0..t {
        case_vec.push(
            to_vec_usize(read_buffer_vec())
        );
        let n: usize = case_vec[h][0];
        for _ in 0..n {
            case_vec.push(to_vec_usize(read_buffer_vec()));
        }
    }

    for h in 0..t {
        let case_vec_now = case_vec[h].clone();
        for i in 0..(case_vec_now.len()-2) {
            let goal_smallest = case_vec_now[1];
            let goal_biggest = case_vec_now[2];
            if case_vec_now[i] <= goal_biggest && case_vec_now[i] >= goal_smallest {

            }
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