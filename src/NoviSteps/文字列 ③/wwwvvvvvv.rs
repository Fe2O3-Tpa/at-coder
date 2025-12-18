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

fn sharp_place(sharp_char: char) -> UInt {
    match sharp_char {
        'v' => 1,
        'w' => 2,
        _ => 0
    }
}

#[allow(unused_doc_comments)]
fn main() {
    /**
     * リンク
     * https://atcoder.jp/contests/abc306/tasks/abc306_a
     *
     * 抽象化
     * for文を回して変数を足していく
     *
     * 必要・十分条件の整理
     * 正しく実装する
     */
    input! {
        s: String
    }
    let s_chars = s.chars().collect::<Vec<char>>();
    let mut output = 0;
    
    rep_as!(i, 0, s_chars.len(), {
        output += sharp_place(s_chars[i]);
    });

    println!("{}", output);
}