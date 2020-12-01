
pub fn bubble_sort<T: std::cmp::Ord>(to_sort: &mut Vec<T>) {
    let mut swapped: bool = false;
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