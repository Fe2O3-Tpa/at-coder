use proconio::input;
use std::cmp::max;

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
     * https://atcoder.jp/contests/abc184/tasks/abc184_b
     *
     * 抽象化
     * 変数(Int)を用意して、Xを代入する。
     * for文で入力をVec<char>で回す
     * maxを代入して変数を変える
     * 
     * 必要・十分条件の整理
     * 正しく実装する
     */
    input! {
        n: UInt,
        x: Int,
        s: String
    }

    let mut point = x;
    let quizzes = s.chars().collect::<Vec<char>>();

    rep_as!(i, 0, n, {
        if quizzes[i] == 'o' {
            point += 1;
        } else {
            point = max(0, point - 1);
        }
    });

    println!("{}", point);
}