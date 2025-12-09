use std::io;

fn main() {
    let input_1: Vec<String> = read_buffer();
    let mut a_string = String::new();
    let mut b_string = String::new();
    let mut c_string = String::new();
    let mut d_string = String::new();

    if let Some(z) = input_1.get(0) {
        a_string = z.to_string();
    }
    if let Some(z) = input_1.get(1) {
        b_string = z.to_string();
    }
    if let Some(z) = input_1.get(2) {
        c_string = z.to_string();
    }
    if let Some(z) = input_1.get(3) {
        d_string = z.to_string();
    }

    let a: i32 = a_string.parse().unwrap();
    let b: i32 = b_string.parse().unwrap();
    let c: i32 = c_string.parse().unwrap();
    let d: i32 = d_string.parse().unwrap();

    if c >= a{
        if d >= b{
            println!("No");
        } else {
            println!("Yes");
        } 
    } else {
        println!("No");
    }
}



fn read_buffer() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split_whitespace().map(|s| s.to_string()).collect()
}