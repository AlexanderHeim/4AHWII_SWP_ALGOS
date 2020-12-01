
#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
mod search;
mod recursion;
mod sort;

use recursion::*;

fn main() {
    println!("{}", fibonacci(10));
}




