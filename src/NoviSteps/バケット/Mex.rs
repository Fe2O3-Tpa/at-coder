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

macro_rules! rep_iter {
    ($iter: expr, $body: block) => {
        for _ in $iter {
            $body
        }
    };
}

macro_rules! rep_iter_as {
    ($var: ident, $iter: expr, $body: block) => {
        for $var in $iter {
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
     * https://atcoder.jp/contests/abc245/tasks/abc245_b
     * 
     * 入力
     * n: 1~2000 Aの要素数
     * A: A_i(1 <= i <= n): 0 ~ 2000
     * 
     * 抽象化や手順
     * Aの各要素をHashSetにぶち込む
     * Aの要素が2000までなので、1から2000まで全部が入っていた場合も2001が答えの最大。
     * for文で0から2001まで回して、存在確認がfalseを返したら変数に束縛してbreakする。
     * その変数を出力する
     * 
     * 必要・十分条件の整理
     */

    input! {
        n: UInt,
        a: [UInt; n]
    }

    let mut set = HashSet::new();
    rep_iter_as!(i, a, {
        set.insert(i);
    });

    let mut output = INF_UINT;
    rep_equal_as!(i, 0, 2001, {
        if !set.contains(&i) {
            output = i;
            break;
        }
    });

    println!("{}", output);
}