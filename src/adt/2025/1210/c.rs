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
        s: String
    }
    let mut ranren: Vec<(char, UInt)> = Vec::new();
    rep_as!(i, 0, s.chars().count(), {
        let ranren_len = ranren.len();
        if ranren_len != 0 {
            if ranren[ranren_len - 1].0 == string_nth(&s, i) {
                ranren[ranren_len - 1].1 += 1;
            } else {
                ranren.push((string_nth(&s, i), 1));
            }
        } else {
            ranren.push((string_nth(&s, i), 1));
        }
    });

    let mut output: UInt = 0;
    rep_as!(i, 0, ranren.len(), {
        if ranren[i].0 == '0' {
            output += (ranren[i].1 / 2) + (ranren[i].1 % 2);
        } else {
            output += ranren[i].1;
        }
    });
    println!("{}", output);
}
