use std::time::{Duration, Instant};
use super::BenchStats;

// Benched Bubble Sort
pub fn benched_bubble_sort(to_sort: &mut [i32]) -> BenchStats {
    let mut stats = BenchStats::new();
    let now = Instant::now();
    let mut swapped;
    loop {
        swapped = false;
        for i in 0..to_sort.len()-1 {
            stats.count_compare_operations += 1;
            if to_sort[i] > to_sort[i+1] {
                stats.count_swap_operations += 1;
                to_sort.swap(i, i+1);
                swapped = true;
            }
        }
        if swapped == false {
            stats.time_taken = now.elapsed();
            return stats;
        }
    }
}

//Simple benching implementation of insertion sort
pub fn benched_insertion_sort(to_sort: &mut [i32]) -> BenchStats {
    let mut stats = BenchStats::new();
    let now = Instant::now();
    for x in 1..to_sort.len() {
        for y in (1..x+1).rev() {
            stats.count_compare_operations += 1;
            if to_sort[y] >= to_sort[y-1] { break; }
            stats.count_swap_operations += 1;
            to_sort.swap(y, y-1);
        }
    }
    stats.time_taken = now.elapsed();
    stats
}

// Benching Implementation of Insertion Sort but using Binary Search to find the correct spot for the value to sort.
pub fn benched_insertion_sort_bin(to_sort: &mut [i32]) -> BenchStats{
    let mut stats = BenchStats::new();
    let now = Instant::now();
    for x in 1..to_sort.len() {

        let mut l: usize = 0;
        let mut r: usize = x-1;
        let mut m: i32;

        loop {

            m = ((l+r)/2) as i32;
            stats.count_compare_operations += 1;
            if l > r {
                break;
            }
            stats.count_compare_operations += 1;
            if to_sort[m as usize] < to_sort[x] {
                l = m as usize+1;
                continue;
            }
            stats.count_compare_operations += 1;
            if to_sort[m as usize] > to_sort[x] {
                stats.count_compare_operations += 1;
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
            stats.count_swap_operations += 1;
            to_sort.swap(y as usize, (y+1) as usize);
        }
    }
    stats.time_taken = now.elapsed();
    stats
}

// Benching Implementation of selection Sort
pub fn benched_selection_sort(to_sort: &mut [i32]) -> BenchStats{
    let mut stats = BenchStats::new();
    let now = Instant::now();
    for i in 0..to_sort.len() {
        let mut lowest = i;
        for j in i..to_sort.len() {
            stats.count_compare_operations += 1;
            if to_sort[j] < to_sort[lowest] {
                lowest = j;
            } 
        }
        stats.count_swap_operations += 1;
        to_sort.swap(i, lowest);
    }
    stats.time_taken = now.elapsed();
    stats
}