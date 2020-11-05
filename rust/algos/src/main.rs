
#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
mod search;
mod recursion;

use recursion::*;

fn main() {
    println!("{}", fibonacci(10));
}




