use num::{BigUint, One};
// use std::mem::replace;

// Calculate large fibonacci numbers.
fn mp(n: BigUint, count: i32) -> i32 {
    println!("{} {}",n, count);
    let digits = n.to_str_radix(10);
    if digits.len() == 1{
        return 1
    }
    let mut f1: BigUint = One::one();
    for d in digits.chars(){
        let x = BigUint::parse_bytes(d.to_string().as_bytes(), 10);
        match x {
            Some(v) => {
                f1=f1*v;
            },
            None => {},
        }
    }
    1+mp(f1, count+1)
}

fn main() {
    let start = BigUint::parse_bytes(b"277777788888899", 10);
    match start {
        Some(v) => {
            mp(v,0);
        },
        None => {},
    }

}
