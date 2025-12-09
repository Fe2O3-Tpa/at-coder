use proconio::input;

type Int = isize;
type UInt = usize;
type Shosuu = f64;
type VecInt = Vec<Int>;
type VecUInt = Vec<UInt>;
type VecString = Vec<String>;
type TwoDim<T> = Vec<Vec<T>>;

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

fn main() {
    input! {
        n: UInt,
        d: UInt,
        s: String
    };
    let mut empty_boxes = 0;
    repeat_as!(i, 0, n, {
        if s.chars().collect::<Vec<char>>()[i] == '.' {
            empty_boxes += 1
        }
    });
    empty_boxes += d;
    println!("{}", empty_boxes);
}
