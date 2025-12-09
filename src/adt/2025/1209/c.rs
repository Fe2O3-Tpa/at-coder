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
        n: UInt
    };

    // グリッドの初期化
    let mut grid = Vec::new();
    repeat_as!(i, 0, n, {
        let hoge = vec![UNKNOWN; n];
        grid.push(hoge);
    });

    repeat_equal_as!(i, 1, n, {
        let j = n + 1 - i;
        if i <= j {
            repeat_as!(y, i-1, j, {
                repeat_as!(x, i-1, j, {
                    if i % 2 == 1 {
                        grid[y][x] = BLACK;
                    } else {
                        grid[y][x] = WHITE;
                    }
                });
            });
        }
    });
    println!("{}", grid.iter().map(|t| t.join("")).join("\n"));
}
