//Fibonacci endrecursive
pub fn fibonacci(index: i32) -> i64 {
    return inner_fibonacci(0, 1, index)
}

fn inner_fibonacci(n1: i64, n2: i64, count: i32) -> i64 {
    if count == 0 {
        return n1;
    }
    inner_fibonacci(n2, n1+n2, count - 1)
}