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

#[allow(unused_doc_comments)]
fn main() {
    /**
     * リンク
     * https://atcoder.jp/contests/adt_easy_20251217_3/tasks/abc381_a
     *
     * 抽象化
     * 実装問題
     *
     * 必要・十分条件の整理
     *
     */
    input! {
        n: UInt,
        s: String
    }

    let mut output = true;

    if n % 2 != 1 {
        output = false;
    }
    rep_as!(i, 0, (n + 1) / 2 - 1, {
        if s.chars().nth(i).unwrap() != '1' {
            output = false;
        }
    });
    if s.chars().nth((n + 1) / 2 - 1).unwrap() != '/' {
        output = false;
    }
    rep_as!(i, (n + 1) / 2, n, {
        if s.chars().nth(i).unwrap() != '2' {
            output = false;
        }
    });

    yes_no(&output);
}
