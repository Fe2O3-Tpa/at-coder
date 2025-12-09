use std::io;

fn main() {
    let s = read_buffer();
    let mut chars_list = s.chars().collect::<Vec<char>>();
    chars_list.sort();
    chars_list.dedup();

    let mut chars_amount = vec![0,0];

    for s_chars in s.chars() {
        for char_place in 0..2 {
            if s_chars == chars_list[char_place] {
                chars_amount[char_place] += 1;
            }
        }
    }
    for i in 0..2 {
        if chars_amount[i] == 1 {
            println!("{}", chars_list[i])
        }
    }
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}