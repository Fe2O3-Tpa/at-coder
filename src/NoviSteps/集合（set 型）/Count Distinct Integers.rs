use std::collections::HashSet;
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

#[allow(unused_doc_comments)]
fn main() {
    /**
     * リンク
     * https://atcoder.jp/contests/abc240/tasks/abc240_b
     * 
     * 入力
     * n: 1~1e3
     * a_i: 1~10e9 かつ 要素数nの数列 (1 <= i <= n)
     * 
     * 抽象化
     * 集合型にぶち込んで個数を出力する
     * 
     * 必要・十分条件の整理
     * 
     */
    input! {
        n: UInt,
        a: [UInt; n]
    }

    let mut set = HashSet::new();
    rep_as!(i, 0, n, {
        set.insert(a[i]);
    });

    println!("{}", set.len());
}