use num::{BigUint, One};
use std::fs::OpenOptions;
use std::io::prelude::*;
// use std::mem::replace;

// Calculate large fibonacci numbers.
fn mp(start: BigUint, n: BigUint, count: i32) -> i32 {
    let digits = n.to_str_radix(10);
    if count >=11{
        println!("{} {} {}",start, n, count);
    }
    if digits.len() == 1{
        // println!("{} {} {}",start, n, count);
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
    1+mp(start,f1, count+1)
}

fn main() {
    /* Make start number with 20K digits as seemingly max already visited*/
    let mut string = String::new();
    string.push_str("2");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("numbers.txt")
        .unwrap();
    for _ in 1..20000{
        string.push_str("2");
    }
    loop{
        let start = BigUint::parse_bytes(string.as_bytes(), 10);
        match start {
            Some(v) => {
                let mut f1: BigUint = One::one();
                f1=&v+f1;
                if !string.contains("0"){
                    if !string.contains("5"){
                        let org = v.clone();
                        if let Err(e) = writeln!(file, "{}", org.to_str_radix(10)) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                        mp(org, v, 0);
                    }
                }
                string = f1.to_str_radix(10);
            },
            None => {},
        }
    }
}
