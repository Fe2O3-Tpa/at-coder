use proconio::input;

type Int = isize;
type UInt = usize;
type BigUint = u128;
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

fn base8_to_base10(base8_value: String) -> BigUint {
    let mut ten_digits = Vec::new();
    for i in 1..=base8_value.len() {
        let eight_digits_i: UInt = base8_value
            .chars()
            .nth(base8_value.len() - i)
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        ten_digits.insert(
            0,
            eight_digits_i * (8_usize.pow((i - 1).try_into().unwrap())),
        );
    }
    ten_digits.iter().sum::<BigUint>()
}

fn base10_to_base9(base10: BigUint) -> String {
    
}

fn main() {
    input! {
        n: String,
        k: UInt
    }
    let mut did_int = n;
    repeat!(0, k, {
        println!("{:?}", ten_digits);
        did_int = base8_to_base10(did_int);

    });
}
