// main.rs
//
// This program provides a detailed, manual benchmark using `std::time::Instant`
// to compare the performance of FFIVector and std::vec::Vec for key operations.
//
// This assumes `FFIVector`, `FFIableObject`, and `Maybe` are available from your crate.

use std::time::Instant;

// Import your custom types from your crate.
// You would replace `my_crate` with the actual name of your crate.
use lrtexec_lib::{common::{others::{boxes::{Boxed, BoxedData}, vector::FFIVector}, FFIableObject}, Maybe};

fn main() {
    const SIZE: i32 = 8000;
    
    // --- Benchmark 1: New from Slice ---
    println!("--- Benchmarking Vector Creation from a slice of size {} ---", SIZE);

    let now_std_new = Instant::now();

    let std_vec_data: Vec<_> = (0..SIZE).map(|x| Boxed::into_raw(Boxed::new( FFIableObject::create_using_box(x)))).collect();
    //let std_vec = std_vec_data.clone();
    let elapsed_std_new = now_std_new.elapsed();

    println!("std::vec::Vec::new completed in: {:?}", elapsed_std_new);

    let ffi_vec_data: Vec<FFIableObject> = (0..SIZE).map(|x| FFIableObject::create_using_box(x as i32)).collect();

    let now_ffi_new = Instant::now();

    let ffi_vec = FFIVector::new(ffi_vec_data).unwrap();
    let elapsed_ffi_new = now_ffi_new.elapsed();
    println!("FFIVector::new completed in: {:?}", elapsed_ffi_new);
    drop(ffi_vec);

    println!("\n");


    // --- Benchmark 2: Push Operations ---
    println!("--- Benchmarking Push Operations for {} items ---", SIZE);

    let now_std_push = Instant::now();
    let mut std_vec_push = Vec::new();
    for i in 0..SIZE {
        std_vec_push.push(i);
    }
    let elapsed_std_push = now_std_push.elapsed();
    println!("std::vec::Vec::push completed in: {:?}", elapsed_std_push);
    drop(std_vec_push);
    
    let now_ffi_push = Instant::now();
    let mut ffi_vec_push = FFIVector::null();
    for i in 0..SIZE {
        ffi_vec_push.push(FFIableObject::create_using_box(i as i32));
    }
    let elapsed_ffi_push = now_ffi_push.elapsed();
    println!("FFIVector::push completed in: {:?}", elapsed_ffi_push);
    drop(ffi_vec_push);

    println!("\n");


    // --- Benchmark 3: Iteration and Summing ---
    println!("--- Benchmarking Iteration and Summing for {} items ---", SIZE);
    let std_vec: Vec<i32> = (0..SIZE).collect();
    let ffi_vec_data: Vec<FFIableObject> = (0..SIZE).map(|x| FFIableObject::create_using_box(x as i32)).collect();
    let ffi_vec = FFIVector::new(ffi_vec_data).unwrap();
    
    let now_std_iter = Instant::now();
    let mut sum_std = 0;
    for &item in std_vec.iter() {
        sum_std += item;
    }
    let elapsed_std_iter = now_std_iter.elapsed();
    println!("std::vec::Vec iteration completed in: {:?}", elapsed_std_iter);
    println!("Resulting sum: {}", sum_std);

    let now_ffi_iter = Instant::now();
    let mut sum_ffi = 0;
    for i in 0..ffi_vec.length() {
        let item = unsafe { ffi_vec.get_at(i..i+1) };
        
        let val = &item[0];
        sum_ffi += unsafe { &*(*val as *const BoxedData<i32>) }.value;
    }
    let elapsed_ffi_iter = now_ffi_iter.elapsed();
    println!("FFIVector iteration completed in: {:?}", elapsed_ffi_iter);
    println!("Resulting sum: {}", sum_ffi);
    
    drop(std_vec);
    drop(ffi_vec);
}
