use std::{io,cmp::max};

fn main() {
    let input1 = read_buffer_vec()
        .iter()
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let n = input1[0];
    let m = input1[1];

    let mut s_seq: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        s_seq.push(
            read_buffer()
                .chars()
                .map(|t| t.to_string().parse().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    let mut human_points: Vec<usize> = Vec::new();
    for _ in 0..n {
        human_points.push(0);
    }

    for vote_nth in 0..m {
        // s_seq[i][j]は人iのj回目の投票
        let mut x = 0;
        for i in 0..n {
            if s_seq[i][vote_nth] == 0 {
                x += 1;
            }
        }

        let mut y = 0;
        for i in 0..n {
            if s_seq[i][vote_nth] == 1 {
                y += 1;
            }
        }

        if x == 0 || y == 0 {
            human_points = human_points.iter().map(|&t| t + 1).collect::<Vec<usize>>();
        } else if x < y {
            for i in 0..n {
                if s_seq[i][vote_nth] == 0 {
                    human_points[i] += 1;
                }
            }
        } else {
            for i in 0..n {
                if s_seq[i][vote_nth] == 1 {
                    human_points[i] += 1;
                }
            }
        }
    }

    let mut max_point: usize = 0;
    for i in 0..n {
        max_point = max(human_points[i], max_point);
    }

    let mut max_point_human: Vec<String> = Vec::new();
    for i in 0..n {
        if human_points[i] == max_point {
            max_point_human.push((i+1).to_string());
        }
    }

    println!("{}", max_point_human.join(" "))
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
