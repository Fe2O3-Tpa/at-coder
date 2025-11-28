use std::io;

fn main() {
    let input = read_buffer();
    let mut x_string = String::new();
    let mut y_string = String::new();
    let mut z_string = String::new();
    
    if let Some(p) = input.get(0){
        x_string = p.to_string();
    }
    if let Some(p) = input.get(1){
        y_string = p.to_string();
    }
    if let Some(p) = input.get(2){
        z_string = p.to_string();
    }

    let x: i32 = x_string.parse().unwrap();
    let y: i32 = y_string.parse().unwrap();
    let z: i32 = z_string.parse().unwrap();
    
    if x==y*z{
        println!("Yes");
    } else if (x-y*z)%(z-1) == 0 && (x-y*z) >= 0 {
        println!("Yes");
    } else {
        println!("No")
    }
}



fn read_buffer() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split_whitespace().map(|s| s.to_string()).collect()
}