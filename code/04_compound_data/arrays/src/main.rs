// Apply the functions
mod utils;

use utils::banner;
use utils::{gen_rand_chars, gen_rand_floats, gen_rand_ints};

fn main() {
    banner("*", 72, "Simple Arrays");
    init_simple_int_array();

    implicit_array_type_inference();

    banner("*", 72, "Generate Random Arrays");
    let size = 10;
    let seed = 42;

    let random_integers = gen_rand_ints(size, seed);
    let random_floats = gen_rand_floats(size, seed);
    let random_characters = gen_rand_chars(size, seed);

    println!("Random integers:\n\t {:?}", random_integers);
    println!("Random floats:\n\t {:?}", random_floats);
    println!("Random characters:\n\t {:?}", random_characters);

    banner("*", 72, "Access arrays elements");
    access_array_elements();

    banner("*", 72, "Slicing arrays");
    slice_arrays();
    println!("{}", "*".repeat(72));
}

// Initialize simple arrays

fn init_simple_int_array() {
    // integer arrays
    let int_arr: [i32; 5] = [1, 2, 3, 4, 5];
    // unsigned array
    let u_arr: [u32; 5] = [11, 22, 33, 44, 55];
    // float array
    let float_arr: [f64; 3] = [3.14, 2.71, 1.99];

    println!("Array of ints:          {:?}", int_arr);
    println!("Array of unsigned ints: {:?}", u_arr);
    println!("Array of float numbers: {:?}", float_arr);
}

// Implicit Type Inference
fn implicit_array_type_inference() {
    // integer arrays
    let int_arr = [1, 2, 3, 4, 5];
    // unsigned array
    let u_arr = [11, 22, 33, 44, 55];
    // float array
    let float_arr = [3.14, 2.71, 1.99];

    println!("Array of ints:          {:?}", int_arr);
    println!("Array of unsigned ints: {:?}", u_arr);
    println!("Array of float numbers: {:?}", float_arr);
}

// Erroneous code
// fn bad_code() {
//     let arr1 = [1, 2, -3];
//     let arr2 = [1, 2.2, 3];
//     let arr3: [u32; 3] = [1, -3, 10];
// }

// Access Array Elements
fn access_array_elements() {
    // Define an array
    let arr: [f64; 10] = gen_rand_floats(10, 0)
        .try_into()
        .expect("Slice with incorrect length");

    // Accessing elements by index
    let first = arr[0];
    let second = arr[1];
    println!("First element: {}", first);
    println!("Second element: {}", second);

    // Accessing the last element using length
    let last = arr[arr.len() - 1];
    println!("Last element: {}", last);

    // Using a loop to access all elements
    for i in 0..arr.len() {
        println!("Element at index {}: {}", i, arr[i]);
    }

    // Using iter() to access all elements
    for (index, &element) in arr.iter().enumerate() {
        println!("Element at index {}: {}", index, element);
    }

    let [first, second, .., last] = arr;
    println!("First: {}, Second: {}, Last: {}", first, second, last);

    // Safe access with get method
    if let Some(element) = arr.get(2) {
        println!("Element at index 2: {}", element);
    } else {
        println!("Index 2 is out of bounds");
    }

    // Attempting to access out of bounds element
    match arr.get(20) {
        Some(element) => println!("Element at index 20: {}", element),
        None => println!("Index 20 is out of bounds"),
    }
}

// slicing arrays
fn slice_arrays() {
    let arr = gen_rand_ints(10, 42);

    // Slice the entire array
    let slice1: &[i32] = &arr[..];
    let slice2 = &arr[..];
    println!("Entire array slice:\n\t {:?}", slice1);
    println!("Entire array slice:\n\t {:?}", slice2);

    // Slice from the beginning to a specific index
    let slice2 = &arr[..5]; // Slices from the start to the 5th index (excluding 5th)
    println!("Slice from start to index 5 (exclusive):\n\t {:?}", slice2);

    // Slice from a specific index to the end
    let slice3 = &arr[5..]; // Slices from the 5th index to the end
    println!("Slice from index 5 to end:\n\t {:?}", slice3);

    // Slice a specific range within the array
    let slice4 = &arr[3..7]; // Slices from the 3rd index to the 7th index (excluding 7th)
    println!("Slice from index 3 to 7 (exclusive):\n\t {:?}", slice4);

    // Slice with inclusive range
    let slice5 = &arr[3..=7]; // Slices from the 3rd index to the 7th index (inclusive)
    println!("Slice from index 3 to 7 (inclusive):\n\t {:?}", slice5);

    // Slice with a range containing a single element
    let slice6 = &arr[4..5]; // Slices only the element at index 4
    println!("Single element slice at index 4:\n\t {:?}", slice6);

    // Slice from start to index (inclusive)
    let slice7 = &arr[..=4]; // Slices from the start to the 4th index (inclusive)
    println!("Slice from start to index 4 (inclusive):\n\t {:?}", slice7);
}
