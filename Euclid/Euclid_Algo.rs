// fn is the function
// function is called gcd 
// parameters are n and m 
// n and m are of type u64 (64 bit integer)
// 4 space indention is standard rust rule 
// loop while with if function
// let declares local variable (t)
// t's type doesn't need to be written out if Rust can infer
// if wanted to spell out: `let t: u64 =m;`
// no return function needed, since block with curly braces is an expression. its value is that of the last expression

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m; m = n; n = t;
}
m = m % n; }
n }