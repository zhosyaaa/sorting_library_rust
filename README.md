
# Sorting Library - Rust

This Rust library supports sorting algorithms such as rapid sort, selection sort, insertion sort, and merge sort.


## Installation

1. Create a new Cargo project by running the following command in your terminal:
   ```bash
   cargo init <name>
   ```

2. Navigate into the newly created `sort` directory:
   ```bash
   cd <name>
   ```

3. Open the `Cargo.toml` file in a text editor and add the following dependencies for your sorting library:
   ```toml
   [dependencies]
   sorting_library = { git = "https://github.com/zhosyaaa/sorting_library_rust.git" }
   ```
Save the `Cargo.toml` file and close the text editor.

4. Import functions from sorting_library in main.rs
   ```main.rs
   use sorting_library::{quick_sort, selection_sort, insertion_sort, merge_sort};
   ```
5. In order to use the functions, declare mutable array of any object type, ex strings, chars, floats, numbers.

```bash
    let mut floats = vec![3.5, 1.2, 5.6, 2.3, 4.7];
    selection_sort(&mut floats);
    println!("Selection Sorted: {:?}", floats);
```
main.rs

## Usage example:
![image](https://github.com/zhosyaaa/sorting_library_rust/assets/123876061/e5782433-a534-4b18-9d01-025f9842c0c7)

Using all functions looks like this:
```main.rs
fn main() {
    // Test with integers
    let mut integers = vec![4, 2, 7, 1, 9, 5];
    println!("Original integers: {:?}", integers);
    
    // Quick sort
    let mut quick_sorted_integers = integers.clone();
    quick_sort(&mut quick_sorted_integers);
    println!("Sorted integers (Quick sort): {:?}", quick_sorted_integers);

    // Selection sort
    let mut selection_sorted_integers = integers.clone();
    selection_sort(&mut selection_sorted_integers);
    println!("Sorted integers (Selection sort): {:?}", selection_sorted_integers);

    // Insertion sort
    let mut insertion_sorted_integers = integers.clone();
    insertion_sort(&mut insertion_sorted_integers);
    println!("Sorted integers (Insertion sort): {:?}", insertion_sorted_integers);

    // Merge sort
    let mut merge_sorted_integers = integers.clone();
    merge_sort(&mut merge_sorted_integers);
    println!("Sorted integers (Merge sort): {:?}", merge_sorted_integers);

    // Test with floats
    let mut floats = vec![4.3, 2.1, 7.8, 1.5, 9.2, 5.7];
    println!("Original floats: {:?}", floats);
    
    // Quick sort
    let mut quick_sorted_floats = floats.clone();
    quick_sort(&mut quick_sorted_floats);
    println!("Sorted floats (Quick sort): {:?}", quick_sorted_floats);

    // Selection sort
    let mut selection_sorted_floats = floats.clone();
    selection_sort(&mut selection_sorted_floats);
    println!("Sorted floats (Selection sort): {:?}", selection_sorted_floats);

    // Insertion sort
    let mut insertion_sorted_floats = floats.clone();
    insertion_sort(&mut insertion_sorted_floats);
    println!("Sorted floats (Insertion sort): {:?}", insertion_sorted_floats);

    // Merge sort
    let mut merge_sorted_floats = floats.clone();
    merge_sort(&mut merge_sorted_floats);
    println!("Sorted floats (Merge sort): {:?}", merge_sorted_floats);

    // Test with strings
    let mut strings = vec!["banana", "apple", "orange", "grape", "kiwi"];
    println!("Original strings: {:?}", strings);
    
    // Quick sort
    let mut quick_sorted_strings = strings.clone();
    quick_sort(&mut quick_sorted_strings);
    println!("Sorted strings (Quick sort): {:?}", quick_sorted_strings);

    // Selection sort
    let mut selection_sorted_strings = strings.clone();
    selection_sort(&mut selection_sorted_strings);
    println!("Sorted strings (Selection sort): {:?}", selection_sorted_strings);

    // Insertion sort
    let mut insertion_sorted_strings = strings.clone();
    insertion_sort(&mut insertion_sorted_strings);
    println!("Sorted strings (Insertion sort): {:?}", insertion_sorted_strings);

    // Merge sort
    let mut merge_sorted_strings = strings.clone();
    merge_sort(&mut merge_sorted_strings);
    println!("Sorted strings (Merge sort): {:?}", merge_sorted_strings);
}
```

To build and run your Rust program, use the following command in the terminal:
!Remember there shouldn't be any Cyrillic or other special characters in your path.
```bash
cargo build
cargo run
```



## License
MIT License

Copyright (c) 2024 Zhansaya Musabekova
