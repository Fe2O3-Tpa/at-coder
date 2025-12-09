use std::{collections::HashSet, io};
use proconio::input;

type Int = isize;
type UInt = usize;
type VecInt = Vec<Int>;
type VecUInt = Vec<UInt>;
type VecString = Vec<String>;
type TwoDim<T> = Vec<Vec<T>>;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: UInt,
    };
    let mut used_username: HashSet<String> = HashSet::new();
    let mut accept_day: Vec<UInt> = Vec::new();
    for i in 1..=n {
        input! {s_i: String};
        if used_username.insert(s_i) {
            accept_day.push(i);
        }
    }
    println!("{}", accept_day.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("\n"));
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}