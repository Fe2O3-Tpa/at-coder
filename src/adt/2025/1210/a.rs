use proconio::input;

type Int = isize;
type UInt = usize;
type BigUint = u128;
type Shosuu = f64;
type VecInt = Vec<Int>;
type VecUInt = Vec<UInt>;
type VecString = Vec<String>;
type TwoDim<T> = Vec<Vec<T>>;

macro_rules! rep {
    ($start:expr, $end:expr, $body:block) => {
        for _ in $start..$end {
            $body
        }
    };
}

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

macro_rules! rep_equ_as {
    ($var: ident, $start:expr, $end:expr, $body:block) => {
        for $var in $start..=$end {
            $body
        }
    };
}

fn string_nth(string: &String, place: UInt) -> char {
    (*string).chars().nth(place).unwrap()
}

#[allow(non_snake_case)]
fn main() {
    input! {
        n: UInt
    }
    let has_ketsuban = if n >= 42 { true } else { false };
    let contest = if has_ketsuban { n + 1 } else { n };
    println!("AGC{contest:0>3}");
}
