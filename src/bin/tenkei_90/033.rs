use proconio::input;

type Int = isize;
type UInt = usize;
type Shosuu = f64;
type VecInt = Vec<Int>;
type VecUInt = Vec<UInt>;
type VecString = Vec<String>;
type TwoDim<T> = Vec<Vec<T>>;

#[allow(non_snake_case)]
fn main() {
    input! {
        input1: [Shosuu; 2]
    }
    let H = input1[0];
    let W = input1[1];

    if H == 1.0 || W == 1.0 {
        println!("{}", (H * W).to_string());
    } else {
        println!("{}", ((H / 2.0).ceil() * (W / 2.0).ceil()).to_string());
    }
}
