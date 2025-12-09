use std::io;

type Int = isize;
type UInt = usize;
type VecUInt = Vec<UInt>;
type TwoDim<T> = Vec<Vec<T>>;

fn main() {
    // 「AがBの約数である」とは、BがAで割り切れるということ。
    // 式にすると、B % A == 0である。
    let n = read_buffer().parse::<UInt>().unwrap();
    let a_seq: VecUInt = read_buffer_vec()
        .iter()
        .map(|t| t.parse().unwrap())
        .collect();
    let mut a_ruisekiwa: VecUInt = vec![0];
    for i in 0..n {
        a_ruisekiwa.insert(i+1,a_ruisekiwa[i] + a_seq[i]);
    }

    let mut output: UInt = 0;
    for l in 1..=n {
        for r in l..=n {
            let ruiseki_sum = a_ruisekiwa[r] - a_ruisekiwa[l-1];
            let isnot_yakusuu = a_seq[(l - 1)..=(r-1)]
                .iter()
                .all(|&a_youso| ruiseki_sum % a_youso != 0);
            if isnot_yakusuu {
                output += 1;
            }
        }
    }
    println!("{}", output.to_string());
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
