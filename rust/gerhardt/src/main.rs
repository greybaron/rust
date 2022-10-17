use std::env::args;
use rug::{Integer, Assign};

fn main() {

    if args().len() > 1 || true {
        for arg_str in args().skip(1) {
            let parsed = parse_int(arg_str);

            // println!("{} → {}", parsed, get_sum(parsed));
            println!("{parsed} → Summe = {}; Factorial = {}", get_sum(parsed), get_factorial(parsed as u128));
        }
    }
    else {
        println!("No argument given");
    }

    
}

fn parse_int(str: String) -> i128 {
    return str.parse().unwrap()
}

fn get_sum(int: i64) -> i64 {
    let mut sum: i64 = 0;
    
    for i in 0..int+1 {
        sum += i;
    }
    return sum;
}

fn get_factorial(int: u128) -> Integer {
    let mut fac = Integer::new();
    fac.assign(1);


    for i in 1..int+1 {
        fac *= i;
    }

    return fac;
}