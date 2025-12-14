use proconio::input;
use std::collections::HashSet;

type Int = isize;
type UInt = usize;
type Shosuu = f64;

const INF_UINT: UInt = 1 << 30;
const INF_INT: Int = 1 << 30;

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

fn yes() {
    println!("Yes");
}

fn no() {
    println!("No");
}

#[allow(unused_doc_comments)]
fn main() {
    /*
     * 抽象化
     *
     * 必要・十分条件の整理
     * MLE対策
     * HashSetを使って使用したマスを保存する
     */
    input! {
        n: UInt,
        m: UInt,
    }
    let mut r = Vec::new();
    let mut c = Vec::new();

    rep_times!(m, {
        input! {
            input: [UInt; 2]
        }
        r.push(input[0]);
        c.push(input[1]);
    });

    let mut block_exist_place_rc: HashSet<(UInt, UInt)> = HashSet::new();
    let mut output: UInt = 0;
    rep_as!(i, 0, m, {
        if !block_exist_place_rc.contains(&(r[i], c[i]))
            && !block_exist_place_rc.contains(&(r[i] + 1, c[i]))
            && !block_exist_place_rc.contains(&(r[i], c[i] + 1))
            && !block_exist_place_rc.contains(&(r[i] + 1, c[i] + 1))
        {
            output += 1;
            block_exist_place_rc.insert((r[i], c[i]));
            block_exist_place_rc.insert((r[i] + 1, c[i]));
            block_exist_place_rc.insert((r[i], c[i] + 1));
            block_exist_place_rc.insert((r[i] + 1, c[i] + 1));
        }
    });
    println!("{}", output);
}
