mod hide;
use crate::hide::a::*;
use crate::hide::b::*;

fn main() {
    let a1: A = A {x: 1,};
    let b1: B = B {x: 0,};
    println!("a1.x={} \nb1.x={}", a1.x, b1.x);
}
