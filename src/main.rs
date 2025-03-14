use std::{ env, process};
use num_bigint::{BigUint, RandBigInt};

fn print_usage_stderr() {
    eprintln!("Usage: hexgen [number-of-bytes]"); 
}

fn main() {
    let a: Vec<_> = env::args().collect();

    if a.len() != 2 {
        print_usage_stderr();
        process::exit(1);
    }

    let Ok(num) = a.last().unwrap().parse::<u8>() else {
        eprintln!("Number of bytes should be a valid u8 value.");
        process::exit(1);
    };

    let bits = num as u32 * 8;

    let pow = BigUint::from(2u32).pow(bits);

    let mut rng = rand::thread_rng();
    let res = rng.gen_biguint_range(&0u32.into(), &pow);
    
    println!("0x{:0width$x}", res, width = num as usize * 2);
}