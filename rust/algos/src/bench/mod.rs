use std::time::{Duration};
use rand::distributions::Uniform;
use rand::distributions::Distribution;
use clap::{Arg,App};

use benched_sort::{benched_bubble_sort, benched_insertion_sort, benched_insertion_sort_bin, benched_selection_sort};

mod benched_sort;

pub struct BenchStats {
    count_compare_operations: u64,
    count_swap_operations: u64,
    time_taken: Duration,
}

impl BenchStats {
    pub fn new() -> Self {
        BenchStats {
            count_swap_operations: 0,
            count_compare_operations: 0,
            time_taken: Duration::default(),
        }
    }
}

impl std::fmt::Display for BenchStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Taken Time: {} milliseconds, Swaps: {}, Compares: {}\n", self.time_taken.as_millis(), self.count_swap_operations, self.count_compare_operations)
    }
}

fn bench(f: &mut dyn FnMut(&mut [i32]) -> BenchStats, to_sort: &mut Vec<Vec<i32>>) -> BenchStats {
    let mut averages = BenchStats::new();
    let mut stats: Vec<BenchStats> = Vec::with_capacity(to_sort.len());
    for i in 0..to_sort.len() {
        stats.push(f(to_sort[i].as_mut()));
    }
    for i in 0..stats.len() {
        averages.count_swap_operations += stats[i].count_swap_operations;
        averages.count_compare_operations += stats[i].count_compare_operations;
        averages.time_taken += stats[i].time_taken;
    }

    averages.count_swap_operations = averages.count_swap_operations/stats.len() as u64;
    averages.count_compare_operations = averages.count_compare_operations/stats.len() as u64;
    averages.time_taken = averages.time_taken/stats.len() as u32;
    averages
}

fn random_vec_batch(amount: usize, size: usize) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = Vec::with_capacity(amount);
    for _ in 0..amount {
        vec.push(random_vec(size));
    }
    vec
}

fn random_vec(length: usize) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::with_capacity(length);
    let mut rng = rand::thread_rng();
    let between = Uniform::from(-1000000..1000000);
    for _ in 0..length {
        vec.push(between.sample(&mut rng));
    }
    vec
}

pub fn main() {
    let matches = App::new("Bench").args(&[
        Arg::with_name("amount")
            .short("a")
            .takes_value(true),
        Arg::with_name("length")
            .short("l")
            .takes_value(true)
    ]).get_matches();

    let mut amount = 5;
    let mut length = 50000;
    
    if matches.value_of("length").is_some() {
        length = matches.value_of("length").unwrap().parse().expect("Length is not an integer!");
    }

    if matches.value_of("amount").is_some() {
        amount = matches.value_of("amount").unwrap().parse().expect("Amount is not an integer!");
    }

    println!("-------------------- BENCHING --------------------");
    let batch = random_vec_batch(amount, length);
    println!("Benching with {} random i32 vectors of length {}\n", batch.len(), batch[0].len());
    
    println!("Benching Bubble Sort..");
    let mut bubble_batch = batch.clone();
    let bubble_stats = bench(&mut benched_bubble_sort, &mut bubble_batch);
    println!("Bubble Sort: {}", bubble_stats);

    println!("Benching Insertion Sort..");
    let mut insertion_batch = batch.clone();
    let insertion_stats = bench(&mut benched_insertion_sort, &mut &mut insertion_batch);
    println!("Insertion Sort: {}", insertion_stats);

    println!("Benching Insertion Binary Sort..");
    let mut insertion_bin_batch = batch.clone();
    let insertion_bin_stats = bench(&mut benched_insertion_sort_bin, &mut insertion_bin_batch);
    println!("Insertion Binary Sort: {}", insertion_bin_stats);

    println!("Benching Selection Sort..");
    let mut selection_batch = batch.clone();
    let selection_stats = bench(&mut benched_selection_sort, &mut &mut selection_batch);
    println!("Selection Sort: {}", selection_stats);

    println!("\nBenching finished!");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}