mod binary_search;
mod bubble_sort;
mod selection_sort;
mod insertion_sort;
mod quick_sort;
mod merge_sort;

use binary_search::binary_search;
use crate::bubble_sort::bubble_sort;
use crate::insertion_sort::insertion_sort;
use crate::merge_sort::merge_sort;
use crate::quick_sort::quick_sort;
use crate::selection_sort::selection_sort;
use std::time::Instant;
use rand::Rng;

fn main() {
    // Generate a large array
    let large_size = 1000;
    let mut large_arr = vec![0; large_size];
    let mut rng = rand::thread_rng();

    for i in 0..large_size {
        large_arr[i] = rng.gen_range(0..100000);
    }

    // Binary Search
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 5;
    let start = Instant::now();

    match binary_search(&arr, target) {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found in the array"),
    }

    let duration = start.elapsed();
    println!("Time elapsed in binary_search: {:?}", duration);

    // Bubble Sort
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    let start = Instant::now();
    bubble_sort(&mut arr);
    let duration = start.elapsed();
    println!("Sorted array (Bubble Sort): {:?}", arr);
    println!("Time elapsed in bubble_sort: {:?}", duration);
    let start = Instant::now();
    bubble_sort(&mut large_arr.clone());
    let duration = start.elapsed();
    println!("Time elapsed in bubble_sort (large array): {:?}", duration);

    // Selection Sort
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    let start = Instant::now();
    selection_sort(&mut arr);
    let duration = start.elapsed();
    println!("Sorted array (Selection Sort): {:?}", arr);
    println!("Time elapsed in selection_sort: {:?}", duration);
    let start = Instant::now();
    selection_sort(&mut large_arr.clone());
    let duration = start.elapsed();
    println!("Time elapsed in selection_sort (large array): {:?}", duration);

    // Insertion Sort
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    let start = Instant::now();
    insertion_sort(&mut arr);
    let duration = start.elapsed();
    println!("Sorted array (Insertion Sort): {:?}", arr);
    println!("Time elapsed in insertion_sort: {:?}", duration);
    let start = Instant::now();
    insertion_sort(&mut large_arr.clone());
    let duration = start.elapsed();
    println!("Time elapsed in insertion_sort (large array): {:?}", duration);

    // Quick Sort
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    let start = Instant::now();
    quick_sort(&mut arr);
    let duration = start.elapsed();
    println!("Sorted array (Quick Sort): {:?}", arr);
    println!("Time elapsed in quick_sort: {:?}", duration);
    let start = Instant::now();
    quick_sort(&mut large_arr.clone());
    let duration = start.elapsed();
    println!("Time elapsed in quick_sort (large array): {:?}", duration);

    // Merge Sort
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    let start = Instant::now();
    merge_sort(&mut arr);
    let duration = start.elapsed();
    println!("Sorted array (Merge Sort): {:?}", arr);
    println!("Time elapsed in merge_sort: {:?}", duration);
    let start = Instant::now();
    merge_sort(&mut large_arr.clone());
    let duration = start.elapsed();
    println!("Time elapsed in merge_sort (large array): {:?}", duration);
}