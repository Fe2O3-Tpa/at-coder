use std::io;

fn main() {
    /* 数の組み合わせの順列全探索(DFS, Depth First Search) */
    let n = read_buffer().parse::<isize>().unwrap();
    let a_seq = read_buffer_vec()
        .iter()
        .map(|t| t.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    if !dfs((1..=n).collect::<Vec<isize>>(), Vec::new(), a_seq)  {
        no();
    }
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}

fn dfs(view_vec: Vec<isize>, now_vec: Vec<isize>, hikaku_seq: Vec<isize>) -> bool {
    // 返り値のboolは即刻returnしてよいか
    let mut output = true;
    if view_vec.len() != 0 {
        for i in 0..view_vec.len() {
            let mut view_vec_copy = view_vec.clone();
            let mut now_vec_copy = now_vec.clone();
            view_vec_copy.remove(i);
            now_vec_copy.push(view_vec[i]);
            output = dfs(view_vec_copy, now_vec_copy, hikaku_seq.clone());
            if output {
                return output;
            }
        }
        return false;
    } else {
        for i in 0..now_vec.len() {
            if !(hikaku_seq[i] == -1 || hikaku_seq[i] == now_vec[i]) {
                output = false;
                break;
            }
        }
        if output {
            println!(
                "Yes\n{}",
                now_vec
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
    }
    return output;
}

fn no() {
    println!("No");
}