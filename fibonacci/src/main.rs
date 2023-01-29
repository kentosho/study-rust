extern crate exercise;

use exercise::fibonacci::*;

fn main() {
    let n = 40;
    println! {"{}-th fibonacci = {} by match",n , fib_match(n)};
    println! {"{}-th fibonacci = {} by regression",n , fib_one(n)};
    println! {"{}-th fibonacci = {} by dyn. prg.",n , fib_dp_simple(n)};
}
