pub fn bubble_sort<T: std::cmp::Ord>(to_sort: &mut [T]) {
    let mut swapped;
    loop {
        swapped = false;
        for i in 0..to_sort.len()-1 {
            if to_sort[i] > to_sort[i+1] {
                to_sort.swap(i, i+1);
                swapped = true;
            }
        }
        if swapped == false {
            return;
        }
    }
}

//Simple implementation of insertion sort
//Written on the 21/12/2020
pub fn insertion_sort<T: std::cmp::Ord>(to_sort: &mut [T]) {
    for x in 1..to_sort.len() {
        for y in (1..x+1).rev() {
            if to_sort[y] >= to_sort[y-1] { break; }
            to_sort.swap(y, y-1);
        }
    }
}

// Implementation of Insertion Sort but using Binary Search to find the correct spot for the value to sort.
// Written on the 21/12/2020
pub fn insertion_sort_bin<T: std::cmp::Ord>(to_sort: &mut [T]) {
    for x in 1..to_sort.len() {

        let mut l: usize = 0;
        let mut r: usize = x-1;
        let mut m: i32;

        loop {

            m = ((l+r)/2) as i32;

            if l > r {
                break;
            }

            if to_sort[m as usize] < to_sort[x] {
                l = m as usize+1;
                continue;
            }

            if to_sort[m as usize] > to_sort[x] {
                if m == 0 { 
                    m = -1;
                    break;
                }
                r = (m-1) as usize;
                continue;
            }

            break;
        
        }

        for y in (m+1..x as i32).rev() {
            to_sort.swap(y as usize, (y+1) as usize);
        }
    }
}

// Implementation of selection Sort
pub fn selection_sort<T: std::cmp::Ord>(to_sort: &mut [T]) {
    for i in 0..to_sort.len() {
        let mut lowest = i;
        for j in i..to_sort.len() {
            if to_sort[j] < to_sort[lowest] {
                lowest = j;
            } 
        }
        to_sort.swap(i, lowest);
    }
}