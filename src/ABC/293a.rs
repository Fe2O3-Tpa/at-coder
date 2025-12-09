use std::io;

fn main() {
    let input1 = read_buffer();
    let mut v: Vec<char> = input1.chars().collect();
    
    for i in 0..(v.len()/2) {
        v.insert(i*2, v[i*2+1].clone());
        v.remove(i*2+2);
    }

    let output: String = v.iter().collect();
    println!("{}", output)
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}