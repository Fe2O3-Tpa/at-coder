use std::collections::HashMap;

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
     * https://atcoder.jp/contests/abc248/tasks/abc248_a
     *
     * 入力
     * s: 数字のみからなる長さ9の文字列
     *
     * 抽象化
     * 連想配列で0から9まで出現回数を記録
     * (sをVec<char>にしてを.map()で.parse()する)
     *
     * 必要・十分条件の整理
     * 正しく実装する
     */

    input! {
        s: String
    }

    let parsed_digits = s
        .chars()
        .map(|t| t.clone().to_string().parse::<UInt>().unwrap())
        .collect::<Vec<UInt>>();

    let mut encounter_amount: HashMap<UInt, UInt> = HashMap::new();
    rep_equal_as!(i, 0, 9, {
        encounter_amount.insert(i, 0);
    });

    rep_iter_as!(i, parsed_digits, {
        encounter_amount.insert(i, encounter_amount.get(&i).unwrap() + 1);
    });

    rep_equal_as!(i, 0, 9, {
        if *encounter_amount.get(&i).unwrap() == 0 {
            println!("{}", i);
            break;
        }
    });
}
