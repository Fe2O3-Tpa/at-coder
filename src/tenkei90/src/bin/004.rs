use std::io;

use itertools::Itertools;
type TwoDim<T> = Vec<Vec<T>>;
type VecUint = Vec<usize>;

fn main() {
    let input1: VecUint = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();
    let h = input1[0];
    let w = input1[1];
    let a_2d_vec = input_2vec(h).iter().map(|t| t.iter().map(|t| t.parse().unwrap()).collect()).collect::<TwoDim<usize>>();
    
    let mut gyo_sum: VecUint = Vec::new();
    let mut retu_sum: VecUint = Vec::new();

    for gyo in 0..h {
        gyo_sum.push(a_2d_vec[gyo].iter().sum::<usize>());
    }
    for retu in 0..w {
        let mut kari = 0;
        for karigyo in 0..h {
            kari += a_2d_vec[karigyo][retu];
        }
        retu_sum.push(kari);
    }
    
    for gyo in 0..h {
        let mut kakugyo_sum_vec: VecUint = Vec::new();
        for retu in 0..w {
            kakugyo_sum_vec.push(gyo_sum[gyo]+retu_sum[retu]-a_2d_vec[gyo][retu]);
        }
        println!("{}",kakugyo_sum_vec.iter().map(|t| t.to_string()).join(" "));
    }
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}

fn input_2vec(tate: usize) -> TwoDim<String> {
    let mut output: TwoDim<String> = Vec::new();
    for _ in 0..tate {
        output.push(read_buffer_vec());
    }
    output
}