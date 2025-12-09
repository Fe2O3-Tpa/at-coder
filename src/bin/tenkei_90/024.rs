use std::io;

type Int = isize;
type UInt = usize;
type VecInt = Vec<Int>;
type VecUInt = Vec<UInt>;
type TwoDim<T> = Vec<Vec<T>>;

fn main() {
    let input1: VecInt = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();
    let n = input1[0];
    let k = input1[1];

    let a: VecInt = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();
    let b: VecInt = read_buffer_vec().iter().map(|t| t.parse().unwrap()).collect();

    let mut diff = 0;
    for i in 0..n as UInt {
        diff += (a[i] - b[i]).abs();
    }

    if k >= diff && (k-diff) % 2 == 0 {
        yes();
    } else {
        no();
    }
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}

fn yes() {
    println!("Yes");
}

fn no() {
    println!("No");
}