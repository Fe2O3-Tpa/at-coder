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
     * https://atcoder.jp/contests/abc408/tasks/abc408_b
     * 
     * 入力
     * n: 1~100
     * A: A_i(1 <= i <= n)が1~100 かつ 要素数nの正整数列
     * 
     * 抽象化
     * HashSetにぶち込む
     * Vecにして昇順ソート
     * 「Vecのlen」と「ソートしたVecを空白区切りしたString」を改行区切りで出力
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

    let mut usize_vec = set.iter().map(|t| (*t).clone()).collect::<Vec<UInt>>();
    // 昇順ソート
    usize_vec.sort();

    let string_vec = usize_vec.iter().map(|t| t.to_string()).collect_vec();

    println!("{}", string_vec.len());
    println!("{}", string_vec.join(" "))
}