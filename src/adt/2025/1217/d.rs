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
     * https://atcoder.jp/contests/adt_easy_20251217_3/tasks/abc390_b
     *
     * 抽象化
     * サンプルから、公比が小数の場合もある。
     * なのでf64で受け取る。
     * Vecに公比を記録する
     * それぞれ同じかどうか調べる
     *
     * 必要・十分条件の整理
     *
     */
    input! {
        n: UInt,
        s: String
    }
    println!("{}",s.replace("na", "nya"));
}
