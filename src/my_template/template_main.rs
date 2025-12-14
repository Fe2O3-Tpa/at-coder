use proconio::input;
use std::cmp::min;

type Int = isize;
type UInt = usize;
type Shosuu = f64;

const INF_UINT: UInt = 1<<30;
const INF_INT: Int = 1<<30;

macro_rules! rep_as {
    ($var: ident, $start:expr, $end:expr, $body:block) => {
        for $var in $start..$end {
            $body
        }
    };
}

macro_rules! rep_as_label {
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
    }
}

fn yes() {
    println!("Yes");
}

fn no() {
    println!("No");
}

#[allow(unused_doc_comments)]
fn main() {
    /**
     * 抽象化
     * 
     * 必要・十分条件の整理
     * 
     */
    
}