use proconio::input;

type Int = isize;
type UInt = usize;
type Shosuu = f64;

const INF_INT: Int = 1 << 30;
const INF_UINT: UInt = 1 << 30;

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
    };
}

fn is_real_kakko(kakko: &String) -> bool {
    let mut count: Int = 0;
    rep_as!(i, 0, (*kakko).len(), {
        if (*kakko).chars().nth(i).unwrap() == '0' {
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

#[allow(unused_doc_comments)]
fn main() {
    /**
     * 抽象化
     * 2^nまでのbit全探索
     * 
     * 必要・十分条件の整理
     * ("("の数) - (")"の数) が途中で負にならない
     * 結果、("("の数) - (")"の数) が0になる
     * TLEしない
     */
    input! {
        n: UInt
    }
    rep_as!(i, 0, 2_usize.pow(n.try_into().unwrap()), {
        let binary_string = format!("{:0>width$b}", i, width = n);
        if binary_string.chars().nth(0).unwrap() == '1' {
            break;
        }
        if is_real_kakko(&binary_string) {
            println!("{}", binary_string.replace('0', "(").replace('1', ")"));
        }
    });
}
