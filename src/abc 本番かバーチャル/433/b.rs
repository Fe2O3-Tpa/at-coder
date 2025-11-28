use std::{fs::read, io};

fn main() {
    let input_1 = read_buffer();
    let mut n_string = String::new();
    
    if let Some(p) = input_1.get(0){
        n_string = p.to_string();
    }

    let n: usize = n_string.parse().unwrap();

    let input_2 = read_buffer();
    for i in [0..n] {
        let a_n_mojiretu = *input_2.get(i).unwrap();
        let a_i = a_n_mojiretu.parse();
        if i == 0 {
            println!("-1");
        } else {
            for h in vec![0..i].reverse() {
                // iはi。
                // hは逆順で身長を選ぶための順番
                // a_sorezoreはAのi以下を選ぶ。
                if let Some(a_sorezore) = input_2.get(h) {
                    if a_sorezore < a_i {
                        println!("{}",a_n_mojiretu);
                    }
                }
            }
        }
    }
}



fn read_buffer() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split_whitespace().map(|s| s.to_string()).collect()
}