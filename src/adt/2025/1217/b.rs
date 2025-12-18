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
     * https://atcoder.jp/contests/adt_easy_20251217_3/tasks/abc227_a
     *
     * 抽象化
     * output(mut)に記録していく
     * for文を抜けたタイミングでoutput + 1を出力する
     *
     * 必要・十分条件の整理
     *
     */
    input! {
        n: UInt,
        k: UInt,
        a: UInt
    }
    let mut now_human = a;

    // 上記の初期化が一個余分なので-1している
    rep_times!(k-1, {
        if now_human == n {
            now_human = 1;
        } else {
            now_human += 1;
        }
    });

    println!("{}", now_human);
}
