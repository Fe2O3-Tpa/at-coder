use proconio::input;

type Int = isize;
type UInt = usize;
type Shosuu = f64;

const INF_UINT: UInt = 1 << 30;
const INF_INT: Int = 1 << 30;
const DIR4: [(Int, Int); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

macro_rules! rep_as {
    ($var: ident, $start:expr, $end:expr, $body:block) => {
        for $var in $start..$end {
            $body
        }
    };
}

macro_rules! rep_equal {
    ($start:expr, $end:expr, $body:block) => {
        for i in $start..=$end {
            let _ = i;
            $body
        }
    };
}

macro_rules! rep_equal_as {
    ($var: ident, $start:expr, $end:expr, $body:block) => {
        for $var in $start..=$end {
            $body
        }
    };
}

macro_rules! rep_times {
    ($times: expr, $body: block) => {
        for _ in 0..$times {
            $body
        }
    };
}

fn yes_no(input: &bool) {
    if *input {
        println!("Yes");
    } else {
        println!("No")
    }
}

fn fizzbuzz(input_int: UInt) -> UInt {
    match (input_int % 3 == 0) || (input_int % 5 == 0) {
        true => 0,
        false => input_int
    }
}

#[allow(unused_doc_comments)]
fn main() {
    /**
     * リンク
     * https://atcoder.jp/contests/abc357/tasks/abc357_b
     *
     * 抽象化
     * 問題文通り
     * 
     * 必要・十分条件の整理
     * 
     */
    input! {
        s: String
    }
    let s_chars = s.chars().collect::<Vec<char>>();
    let mut lowercase_amount = 0;
    let mut uppercase_amount = 0;

    rep_as!(i, 0, s_chars.len(), {
        if s_chars[i].is_lowercase() {
            lowercase_amount += 1;
        } else if s_chars[i].is_uppercase() {
            uppercase_amount += 1;
        }
    });

    let mut output_str = String::new();
    if uppercase_amount > lowercase_amount {
        output_str = s.to_uppercase();
    } else {
        output_str = s.to_lowercase();
    }

    println!("{}", output_str);
}