use std::io;

fn main() {
    let mut query_list: Vec<Vec<String>> = Vec::new();
    for _ in [0,1] {
        query_list.push(
            read_buffer()
                .chars()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
        );
    }
    if how_far(&query_list[0][0], &query_list[0][1]) == how_far(&query_list[1][0], &query_list[1][1]) {
        println!("Yes");
    } else {
        println!("No")
    }
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}

fn how_far(s1: &str, s2: &str) -> usize {
    if s1 == "A" {
        if ["B", "E"].contains(&s2) { 1 } else { 2 }
    } else if s1 == "B" {
        if ["A", "C"].contains(&s2) { 1 } else { 2 }
    } else if s1 == "C" {
        if ["B", "D"].contains(&s2) { 1 } else { 2 }
    } else if s1 == "D" {
        if ["C", "E"].contains(&s2) { 1 } else { 2 }
    } else {
        if ["A", "D"].contains(&s2) { 1 } else { 2 }
    }
}
