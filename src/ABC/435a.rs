use std::io;

type Int = isize;
type UInt = usize;
type VecUInt = Vec<UInt>;
type TwoDim<T> = Vec<Vec<T>>;

fn main() {
    let n = read_buffer().parse::<UInt>().unwrap();
    let mut output = 0;
    for i in 1..=n {
        output += i;
    }
    println!("{}", output);
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