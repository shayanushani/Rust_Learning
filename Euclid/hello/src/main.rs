// use = declaration, Write/FromStr = traits
// trait = collection of methods a type can implement 
// write equates to writeln
// fromstr equates to from_str


fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m; m = n; n = t;
}
m = m % n; }
n }

use std::io::Write;
use std::str::FromStr;

    // mutable local variable, initialized to an empty vector. Infered Vec<u64>
fn main() {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
                     .expect("error parsing argument"));
}

    // Iterator - a value producing each argument in demand
    // iterators are ubiquitous in Rust 

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
}

    // Check that vector has at least one element, and exit if not
    // A "slice" of our vector. We're not "taking ownership" we're "borrowing"

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }


    // output stream
    // Rust assumes returned output means success, while explicitly calling using expect/exit to terminate with error status code means unsuccessful
    println!("The greatest common divisor of {:?} is {}",
    numbers, d);
}