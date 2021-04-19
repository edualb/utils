fn main() {
    references_and_borrowing();
}

// 1.
// References and Borrowing
//
// Notice that all tuple code in the variable declaration and the function return values is gone.
// Second, note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String.
//
// These ampersand(&) are references, and they allow you to refer to some value without taking ownership of it.
//
// Memory representation:
//
// s -> | name | value           s1 -> | name      | value |             H1 -> | index | value
//        ptr  | --------------------->  ptr       | -----------------------> 0   | h
//                                       len       | 5                        1   | e
//                                       capacity  | 5                        2   | l
//                                                                            3   | l
//                                                                            ... | ...
//
// So, due s does not own it, the value it points(&s1) to will not be dropped when the reference goes out of scope
//
// We call having references as function parameters borrowing.
fn references_and_borrowing() {
    // s1 here
    let s1 = String::from("Hello");

    let len = calculate_length2(&s1);

    println!("The length of '{}', is {}", s1, len);
}

// s here
fn calculate_length2(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

// If you try to use this function it won't work because you are trying to modify a borrow parameter.
fn change(some_string: &String) {
    some_string.push_str(", world");
}

//
// 2.
// Mutable References
//
// To fix the last code, we need to use the code below.
//
// We had to change s to be mut. Then we had to create a mutable reference 
// with &mut s and accept a mutable reference with some_string: &mut String
fn mutable_references() {
    let mut s = String::from("hello");

    change2(&mut s);
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

//
// ----------------------------------------------------------------------------------------------------------
//
// Mutable references have one big restriction: 
// You can have only one mutable reference to a particular piece of data in a particular scope.
// The code below will fail.
//
// This restriction allows for mutation but in a very controlled fashion.
// The benefit of having this restriction is that Rust can prevent data races at compile time.
// Data race is similar to a race condition and happens when these behaviors occur:
//
//  * Two or more pointers access the same data at the same time
//  * At least one of the pointers is being used to write to the data.
//  * There is no mechanism being used to synchronize access to the data.
//
// Data races cause undefines behavior and can be difficult to diagnose and fix when you are trying to track them down at runtime.
//
fn example_references_and_borrowing() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // cannot borrow 's' as mutable more than once at a time

    println!("{}, {}", r1, r2);
}

//
// ----------------------------------------------------------------------------------------------------------
//
// A similar rule exists for combining mutable and immutable references...
//
// Error: Cannot borrow 's' as mutable because it is also borrowed as immutable
fn example_references_and_borrowing2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
}

//
// ----------------------------------------------------------------------------------------------------------
//
// Dangling References
//
// In Rust the compiler guarantees that references (freeing  some memory while preserving a pointer to that memory) 
// will never be dangling references: If you have a reference to some data, the compiler will ensure that the data 
// will not go out of scope before the reference to the data does.
//
// Trying create dangling reference in Rust:
//
fn dangling_references() {
    let reference_to_nothing = dangle();
}

// We will discover lifetime error later
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away. !!!DANGER!!!

// Solution:
fn dangle_solution() -> String {
    let s = String::from("hello");
    s
}