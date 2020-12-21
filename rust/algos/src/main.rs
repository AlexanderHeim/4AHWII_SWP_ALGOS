
#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
mod search;
mod recursion;
mod sort;

use sort::*;
use search::binary_search_t1;

fn main() {
    let mut to_sort = [1, 2, 3, 0];
    insertion_sort_bin(&mut to_sort);
    println!("{:?}", to_sort);
}




