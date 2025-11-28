use std::io;

fn main() {
    let x: usize = read_buffer().parse().unwrap();
    let n: usize = read_buffer().parse().unwrap();
    let w_sequence: Vec<usize> = read_buffer_vec().iter().map(|x| x.parse().unwrap()).collect();
    let q: usize = read_buffer().parse().unwrap();
    
    let mut p_sequence: Vec<usize> = vec![];
    for _ in 0..q {
        let add: usize = read_buffer().parse().unwrap();
        p_sequence.push(add);
    }

    let mut has_buhin: Vec<bool> = Vec::new();
    for _ in 0..n {
        has_buhin.push(false);
    }

    for i in 0..q {
        has_buhin[p_sequence[i]-1] = !has_buhin[p_sequence[i]-1];
        let mut buhin_sum: usize = 0;
        for i in 0..n {
            if has_buhin[i] {
                let w_i: usize = w_sequence[i];
                buhin_sum += w_i;
            }
        }
        println!("{}", x+buhin_sum);
    }
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}