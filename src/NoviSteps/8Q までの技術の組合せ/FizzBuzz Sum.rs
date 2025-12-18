use proconio::input;

type Int = isize;
type UInt = u64;
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

fn fizzbuzz(input_int: UInt) -> UInt {
    match (input_int % 3 == 0) || (input_int % 5 == 0) {
        true => 0,
        false => input_int
    }
}

#[allow(unused_doc_comments)]
fn main() {
    /**
     * リンク
     * https://atcoder.jp/contests/abc162/tasks/abc162_b
     *
     * 抽象化
     * 文字列であるFizzやBuzz、FizzBuzzは無視して良い。
     * forループと関数、match式で総和を計算する。
     * 
     * 必要・十分条件の整理
     * こんな感じの関数を定義してforで回す。
     * i % 3 || i % 5が成り立てば無視する。そうでない場合、iを返す
     */
    input! {
        i: UInt
    }

    let mut sum = 0;
    rep_equal_as!(i, 1, i, {
        sum += fizzbuzz(i);
    });

    println!("{}", sum);
}