mod hide;
use crate::hide::{A, B};  

fn main() {
    let a: A = A {x: 1,};
    let b: B = B {x: 0,};
    println!("a.x={} \nb.x={}", a.x, b.x);
}
