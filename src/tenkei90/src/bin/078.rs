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

fn main() {
    input! {
        n: UInt,
        m: UInt
    }

    let mut a: Vec<UInt> = Vec::new();
    let mut b: Vec<UInt> = Vec::new();

    repeat!(0,m,{
        input! {input: [UInt; 2]}
        a.push(input[0]-1);
        b.push(input[1]-1);
    });

    // graphの頂点1は頂点0として表される
    let mut graph: Vec<VecUInt> = vec![vec![]; n];

    repeat_as!(i, 0, m, {
        graph[a[i]].push(b[i]);
        graph[b[i]].push(a[i]);
    });

    let mut output: UInt = 0;
    repeat_as!(i, 0, n, {
        if graph[i].iter().filter(|&&t| t < i).collect::<Vec<&UInt>>().len() == 1 {
            output += 1;
        }
    });
    println!("{}", output);
}