use std::io;

fn main() {
    let input_vec = read_buffer();
    let mut n_string =&String::new(); 
    let mut x_string =&String::new(); 
    let mut mojiretu = String::new();

    if let Some(z) = input_vec.get(0){
        n_string = z;
    }
    if let Some(z) = input_vec.get(1){
        x_string = z;
    }

    let n: i32 = (*n_string).parse().unwrap();
    let x: i32 = (*x_string).parse().unwrap();

    let n_usize: usize = n as usize;
    let x_usize: usize = x as usize;

    for character in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars(){
        mojiretu += &(character.to_string()).repeat(n_usize);
    }
    println!("{}",mojiretu.chars().nth(x_usize-1).unwrap());
}

fn read_buffer() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().split_whitespace().map(|s| s.to_string()).collect()
}
