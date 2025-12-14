use proconio::input;
use std::cmp::min;

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
     * 抽象化(解説)
     * 
     * 
     * 必要・十分条件の整理
     */
    input! {
        n: UInt,
        q: UInt,
        a: [Int; n],
        b: [Int; n],
    }
    
    let mut a_clone = a.clone(); 
    let mut b_clone = b.clone();

    // それぞれのminをベクタにしてまとめる
    let mut min_ab = Vec::new();
    rep_as!(i, 0, n, {
        min_ab.push(min(a[i], b[i]));
    });
    let mut ab_sum = min_ab.iter().sum::<Int>();

    rep_times!(q, {
        input! {
            c_i: char,
            mut x_i: UInt,
            v_i: Int
        }
        // 0-basedに修正
        x_i -= 1;
        
        // クエリの処理
        let mut diff: Int = 0;
        if c_i == 'A' {
            // どれだけ変化したかをdiffに保存
            diff = min(v_i, b_clone[x_i]) - min_ab[x_i];
            
            // それぞれ保存
            a_clone[x_i] = v_i;
            min_ab[x_i] = min(a_clone[x_i], b_clone[x_i]);
        } else {
            diff = min(a_clone[x_i], v_i) - min_ab[x_i];
            b_clone[x_i] = v_i;
            min_ab[x_i] = min(a_clone[x_i], b_clone[x_i]);
        }

        // minの総和にどれだけ変化したか(diff)を足して、総和を計算する
        ab_sum += diff;
        println!("{}", ab_sum);
    });
}