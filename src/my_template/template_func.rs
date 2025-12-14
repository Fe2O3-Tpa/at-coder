use proconio::input;

type Int = isize;
type UInt = usize;
type Shosuu = f64;

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

macro_rules! rep_times {
    ($times: expr, $body: block) => {
        for _ in 0..$times {
            $body
        }
    }
}

fn string_nth(string: &String, place: UInt) -> char {
    (*string).chars().nth(place).unwrap()
}

fn run_length(input: String) -> Vec<(char, UInt)> {
    let mut output: Vec<(char, UInt)> = Vec::new();
    rep_as!(i, 0, input.chars().count(), {
        let output_len = output.len();
        if output_len != 0 {
            if output[output_len - 1].0 == string_nth(&s, i) {
                output[output_len - 1].1 += 1;
            } else {
                output.push((string_nth(&s, i), 1));
            }
        } else {
            output.push((string_nth(&s, i), 1));
        }
    });
    output
}

fn main() {
    
}