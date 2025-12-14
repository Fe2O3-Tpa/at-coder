use proconio::input;

type Int = isize;
type UInt = usize;
type Shosuu = f64;

const INF_UINT: UInt = 1<<30;
const INF_INT: Int = 1<<30;
const BLANK: Int = -1;

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
    /*
     * 抽象化
     * n行n列の配列を作る(UNKNOWNで埋める)
     * 前回代入したマスを記録(毎回)
     * 0,(n-1)/2に1を代入
     * rep_times(n.pow(2)-1)で以下を繰り返す
     * そのまま実装
     * 
     * 必要・十分条件の整理
     * 
     */
    input! {
        n: UInt
    }
    let mut grid = vec![vec![BLANK; n]; n];
    // 初手の操作
    grid[0][(n-1)/2] = 1;
    let mut last_masu: (UInt, UInt) = (0, (n-1)/2);
    let mut last_int: UInt = 1;
    // 2の操作
    rep_times!(n.pow(2)-1, {
        let r = last_masu.0;
        let c = last_masu.1;
        // めんどくさいmod処理
        let option1 = if ((r as Int)-1) >= 0 {
            ((((r as Int)-1) % (n as Int)).abs() as UInt, (c+1) % n)
        } else {
            (((n as Int) + ((r as Int)-1)).abs() as UInt, (c+1) % n)
        };
        let option2 = ((r+1) % n, c);
        if grid[option1.0][option1.1] == BLANK {
            grid[option1.0][option1.1] = (last_int as Int) + 1;
            last_masu = option1;
        } else {
            grid[option2.0][option2.1] = (last_int as Int) + 1;
            last_masu = option2;
        }
        last_int += 1;
    });
    
    rep_as!(i, 0, grid.len(), {
        println!("{}", grid[i].iter().map(|t| t.to_string()).collect::<Vec<String>>().join(" "));
    });
}