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
    // mt_cardの上の方を小さい値にする
    let mut output: VecString = Vec::new();
    let mut mt_card: VecUInt = Vec::new();

    input! {q: UInt};
    repeat!(0, q, {
        input! {input: [UInt; 2]};
        let t_i = input[0];
        let x_i = input[1];
        if t_i == 1 {
            mt_card.insert(0, x_i);
        } else if t_i == 2 {
            mt_card.insert(mt_card.len(), x_i);
        } else {
            output.push(mt_card[x_i-1].to_string());
        }
    });

    println!("{}", output.join("\n"));
}