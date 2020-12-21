
pub fn sequencial_search(vector: &Vec<i32>, number: i32) -> Option<usize> {
    for x in 0..vector.len() {
        if vector[x] == number {
            return Some(x);
        }
    }
    None
}

pub fn binary_search<T: std::cmp::Ord>(vector: &[T], number: &T) -> Option<usize> {

    let mut l: usize = 0;
    let mut r: usize = vector.len()-1;

    loop {

        if l > r {
            return None;
        }

        let m = (l+r)/2;

        if vector[m] < *number {
            l = m+1;
            continue;
        }

        if vector[m] > *number {
            r = m-1;
            continue;
        }

        return Some(m)
        
    }
}

pub fn binary_search_t1<T: std::cmp::Ord>(vector: &[T], number: &T) -> Option<usize> {

    let mut l: usize = 0;
    let mut r: usize = vector.len()-1;

    loop {

        let m = (l+r)/2;

        if l > r {
            return Some(m);
        }

        if vector[m] < *number {
            l = m+1;
            continue;
        }

        if vector[m] > *number {
            r = m-1;
            continue;
        }

        return Some(m)
        
    }
}