use itertools::Itertools;
use proconio::input;

type Int = isize;
type UInt = usize;
type Shosuu = f64;
type VecInt = Vec<Int>;
type VecUInt = Vec<UInt>;
type VecString = Vec<String>;
type TwoDim<T> = Vec<Vec<T>>;

const BLACK: &str = "#";
const WHITE: &str = ".";
const UNKNOWN: &str = "?";

macro_rules! repeat {
    ($start:expr, $end:expr, $body:block) => {
        for _ in $start..$end {
            $body
        }
    };
}

macro_rules! repeat_as {
    ($var: ident, $start:expr, $end:expr, $body:block) => {
        for $var in $start..$end {
            $body
        }
    };
}

macro_rules! repeat_equal {
    ($start:expr, $end:expr, $body:block) => {
        for i in $start..=$end {
            let _ = i;
            $body
        }
    };
}

macro_rules! repeat_equal_as {
    ($var: ident, $start:expr, $end:expr, $body:block) => {
        for $var in $start..=$end {
            $body
        }
    };
}

fn main() {
    input! {
        n: UInt,
        p: UInt,
        a: [UInt; n]
    };

    let mut output: UInt = 0;
    repeat_as!(i, 0, n, {
        if a[i] < p {
            output += 1;
        }
    });
    println!("{}", output)
}