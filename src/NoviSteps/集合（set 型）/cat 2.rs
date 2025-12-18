use std::collections::HashSet;
use itertools::Itertools;
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
     * https://atcoder.jp/contests/abc413/tasks/abc413_b
     * 
     * 入力
     * n: 2~100
     * s: 英小文字の文字列 かつ 長さが1~10
     * 
     * 抽象化
     * i,jをnで2重ループし、S_iとS_jを連結した文字列をHashSetにぶち込む
     * (ただし、iとjは相異なる。つまりif i != jで実行)
     * 
     * 必要・十分条件の整理
     * 問題文通り実装する。
     */
    input! {
        n: UInt,
        s: [String; n]
    }

    let mut set = HashSet::new();
    rep_as!(i, 0, n, {
        rep_as!(j, 0, n, {
            if i != j {
                let s_comb = format!("{}{}", s[i], s[j]);
                set.insert(s_comb);
            }
        });
    });

    println!("{}", set.len());
}