use proconio::input;

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
     * 1番目のクエリは、
     * 「2つのAを連結した整数列B」と、
     * 「2番目のクエリでのlとrに『これまでのcの総和』を足す」ことで求められる。
     * あと累積和。
     *
     * 必要・十分条件の整理
     * 正しく実装する。
     *
     */
    input! {
        n: UInt,
        q: UInt,
        a: [UInt; n]
    }
    let mut b: Vec<UInt> = Vec::new();
    rep_times!(2, {
        rep_as!(i, 0, n, {
            b.push(a[i]);
        });
    });

    // bの累積和を求める
    let mut b_ruisekiwa: Vec<UInt> = vec![0];
    rep_as!(i, 0, 2 * n, {
        b_ruisekiwa.push(*(b_ruisekiwa.last().unwrap()) + b[i])
    });

    // クエリに対する答えを記録する
    let mut c_sum = 0;
    let mut output_uint = Vec::new();
    rep_times!(q, {
        input! {
            query_type: UInt
        }
        if query_type == 1 {
            input! {
                c: UInt
            }
            c_sum += c;
        } else {
            input! {
                l: UInt,
                r: UInt
            }
            output_uint.push(b_ruisekiwa[r + (c_sum % n)] - b_ruisekiwa[l - 1 + (c_sum % n)]);
        }
    });

    let output = output_uint
        .iter()
        .map(|&t| t.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", output);
}
