#[allow(unused_imports)]
use rayon::prelude::*;

fn quicksort_parallel<T: Ord + Send>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let pivot_index = partition(data);
    let (left, right) = data.split_at_mut(pivot_index);

    rayon::join(|| quicksort_parallel(left), || quicksort_parallel(right));
}

fn partition<T: Ord + Send>(data: &mut [T]) -> usize {
    let pivot_index = data.len() / 2;
    data.swap(pivot_index, data.len() - 1);

    let mut i = 0;
    for j in 0..data.len() - 1 {
        if data[j] <= data[data.len() - 1] {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, data.len() - 1);
    i
}

fn partition_single_threaded<T: Ord>(data: &mut [T]) -> usize {
    let pivot_index = data.len() / 2;
    data.swap(pivot_index, data.len() - 1);

    let mut i = 0;
    for j in 0..data.len() - 1 {
        if data[j] <= data[data.len() - 1] {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, data.len() - 1);
    i
}

fn quicksort_single_threaded<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let pivot_index = partition_single_threaded(data);
    let (left, right) = data.split_at_mut(pivot_index);

    quicksort_single_threaded(left);
    quicksort_single_threaded(&mut right[1..]);
}

fn main() {
    let n = 99999; // Adjust the size of the vector here
    let mut data_parallel: Vec<u32> = (0..n).rev().collect(); // Creating a vector with large data in reverse order
    let mut data_single_threaded = data_parallel.clone();

    // for val in data_parallel.iter() {
    //     println!("val: {}", val);
    // }


    // Rayon's doc code:
    let mut v = [-5i32, 4, 1, -3, 87, 3, 2, -18];

    println!("Sorting with Rayon docs code (parallel)...");
    let start_rayon_doc_code = std::time::Instant::now();
    v.par_sort_by(|a, b| a.cmp(b));
    
    let duration_rayon_doc_code = start_rayon_doc_code.elapsed();
    println!("Rayon (parallel) took: {:?}\n", duration_rayon_doc_code);

    for val in v.iter() {
        println!("v elm: {}", val);
    }
    println!();


    println!("Sorting with Rayon (parallel)...");
    let start_parallel = std::time::Instant::now();
    quicksort_parallel(&mut data_parallel);
    let duration_parallel = start_parallel.elapsed();
    println!("Rayon (parallel) took: {:?}\n", duration_parallel);

    println!("Sorting with single thread...");
    let start_single_threaded = std::time::Instant::now();
    quicksort_single_threaded(&mut data_single_threaded);
    let duration_single_threaded = start_single_threaded.elapsed();
    println!("Single-threaded took: {:?}\n", duration_single_threaded);
}


// https://docs.rs/rayon/1.2.0/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_unstable
#[test]
fn rayon_docs_code_quicksort_by_ascending() {

    let mut v = [-5i32, 4, 1, -3, 87, 3, 2, -18];

    v.par_sort_by(|a, b| a.cmp(b)); 

    assert_eq!(v, [-18, -5, -3, 1, 2, 3, 4, 87]);
}

#[test]
fn rayon_docs_code_quicksort_by_descending() {

    // reverse sorting
    let mut v = [-5i32, 4, 1, -3, 87, 3, 2, -18];

    v.par_sort_by(|a, b| b.cmp(a));

    assert_eq!(v, [87, 4, 3, 2, 1, -3, -5, -18]);
}

#[test]
fn rayon_docs_code_quicksort_by_key_abs() {

    let mut v = [-5i32, 4, 1, -3, 87, 3, 2, -18];

    v.par_sort_by_key(|k| k.abs());

    assert_eq!(v, [1, 2, -3, 3, 4, -5, -18, 87]);
}