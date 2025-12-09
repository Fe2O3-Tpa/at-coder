use std::{cmp::max, io};

fn main() {
    // 標準入力を受け取る
    let input1: Vec<usize> = read_buffer_vec()
        .iter()
        .map(|t| t.parse().unwrap())
        .collect();
    let n = input1[0];
    let k = input1[1];
    let s = read_buffer();

    // t_count_vecと照らし合わせる用のすべてのtを記録。
    // ここでは初期化。
    let mut t_vec: Vec<String> = Vec::new();

    // x、つまり出力を決めるための準備。tが出てくる数を記録。
    // （t_vecと照らし合わせる）
    // ここでは初期化。
    let mut t_count_vec: Vec<usize> = Vec::new();
    for _ in 0..=n-k {
        t_count_vec.push(0);
    }

    // tの出現回数をt_count_vecに記録。
    for i in 0..=n-k {
        let t = &s[i..i+k];
        t_vec.push(t.to_string());
        t_count_vec[
            t_vec
                .iter()
                .position(|x| x == t)
                .unwrap()
        ] += 1;
    }

    // t_count_vecの最大値を記録。
    let mut t_len_count_max: usize = 0;
    for i in 0..t_vec.len() {
        t_len_count_max = max(t_len_count_max, t_count_vec[i]);
    }

    // 出現回数が最大となるような長さKの文字列(t)を新しいVecにフィルターする。
    let mut t_max_filtered: Vec<String> = Vec::new();
    for i in 0..t_vec.len() {
        if t_count_vec[i] == t_len_count_max {
            t_max_filtered.push(t_vec[i].clone());
        }
    }

    // 辞書式昇順
    t_max_filtered.sort();

    println!("{}", t_len_count_max.to_string());
    println!("{}", t_max_filtered.join(" "))
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