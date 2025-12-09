use std::io;

type VecUint = Vec<usize>;
type TwoDim<T> = Vec<Vec<T>>;

fn main() {
    let n = read_buffer().parse::<usize>().unwrap();
    let mut c_seq: VecUint = Vec::new();
    let mut p_seq: VecUint = Vec::new();

    for _ in 0..n {
        let hoge: VecUint = read_buffer_vec()
            .iter()
            .map(|t| t.parse().unwrap())
            .collect();
        c_seq.push(hoge[0]);
        p_seq.push(hoge[1]);
    }

    let q: usize = read_buffer().parse::<usize>().unwrap();

    let mut lr_input: TwoDim<usize> = Vec::new();
    for _ in 0..q {
        lr_input.push(
            read_buffer_vec()
                .iter()
                .map(|t| t.parse().unwrap())
                .collect(),
        );
    }

    let mut point_ruisekiwa_1class: VecUint = vec![0];
    for i in 0..n {
        point_ruisekiwa_1class
            .push(point_ruisekiwa_1class[i] + if c_seq[i] == 1 { p_seq[i] } else { 0 });
    }

    let mut point_ruisekiwa_2class: VecUint = vec![0];
    for i in 0..n {
        point_ruisekiwa_2class
            .push(point_ruisekiwa_2class[i] + if c_seq[i] == 2 { p_seq[i] } else { 0 });
    }

    for i in 0..q {
        let l = lr_input[i][0];
        let r = lr_input[i][1];

        let point_slice_sum_1class = point_ruisekiwa_1class[r] - point_ruisekiwa_1class[l-1];
        let point_slice_sum_2class = point_ruisekiwa_2class[r] - point_ruisekiwa_2class[l-1];

        println!("{} {}", point_slice_sum_1class, point_slice_sum_2class);
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
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}