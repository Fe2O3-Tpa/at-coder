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

fn num_to_binary(num: &UInt) -> String {
    format!("{:b}", *num)
}

fn is_good_kakko(input: &String) -> bool {
    let mut count: Int = 0;
    rep_as!(i, 0, (*input).chars().count(), {
        if (*input).chars().collect::<Vec<char>>()[i] == '(' {
            count += 1;
        } else {
            count -= 1;
        }
        if count < 0 {
            return false;
        }
    });
    if count == 0 {
        true
    } else {
        false
    }
}

fn yes() {
    println!("Yes");
}

fn no() {
    println!("No");
}

fn string_nth(string: &String, place: UInt) -> char {
    (*string).chars().nth(place).unwrap()
}

fn main() {
    input! {
        a: UInt,
        b: UInt,
        c: UInt,
        d: UInt
    }
    if c > a {
        no();
    } else if c < a {
        yes();
    } else { // c == a の場合
        if d > b {
            no();
        } else {
            yes();
        }
    }
}