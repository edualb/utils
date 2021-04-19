// use std::io::{self};
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    println!("{}", "hello")
}

// 1.
// Two importants points:
// 1. When s comes into scope, it is valid
// 2. It remains valid until it goes out of scope
fn variable_scope() { // s is not valid here; it's not yet declared
    let s = "hello"; // s is valid from this point forward

    // do stuffs with s
} // this scope is now over, and s is no longer valid





// 2.
// The String Type | Memory and Allocation
//
// Why can String be mutated but literals cannot? The difference is how these two types deal with memory
// In many languages the garbage collector (GC) keeps track and cleans up memory that isn't being used anymore, and we do nor need to think about it.
// Without a GC, it is our responsability to identify when memory is no longer being used and call code to explicitly return it, just as we did to request it.
//
// Doing this correctly has historically been a difficult programming problem:
//  * If we forget, we will waste memory
//  * If we do it too early, we will have an invalid variable.
//  * If we do it twice, that is a bug too
//
// Rust takes a different approach: The memory is automatically returned once the variable that owns it goes out of scope. memory_ad_allocation is an example using a String instead of a string literal
fn string_type() {
    let mut s = String::from("hello"); // The memory was requested by operating system at runtime

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // this will print `hello, world!`
}

fn memory_and_allocation() {
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
} // this scope is now over, and s is no longer valid. Here Rust returns the function called drop. Similar to RAII pattern.





// 3.
// Ways that variable and Data Interact: Move
//
fn way_move() {
    let x = 5; // bind the value 5 to x (pushed on to the stack)
    let y = x;  // copye the value of x(5) and bind to y (pushed on to the stack)


    // s1 -> | name      | value |              H1 ->| index | value
    //         ptr       | -----------------------> 0   | h
    //         len       | 5                        1   | e
    //         capacity  | 5                        2   | l
    //                                              3   | l
    //                                              ... | ...
    //
    // Left side -> A String is made up of three parts:
    //   * pointer: A pointer to the memory that holds the contents of the string;
    //   * length: Is how much memory, in bytes, the contents of the String is currently using.
    //   * capacity: Is the total amount of memory, in bytes, that the String has received from the operating system.
    //
    // Right side -> The memory on the heap that holds the contents.
    //
    //
    let s1 = String::from("Hello");
    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length and the capacity.
    // If we considere the last represetantion (H1), will be something like that:
    //
    // s1 -> | name      | value |
    //       | ptr       | H1    |
    //       | len       | 5     |
    //       | capacity  | 5     |
    //
    // s2 -> | name      | value |
    //       | ptr       | H1    |
    //       | len       | 5     |
    //       | capacity  | 5     |
    //
    // But, what really happens in rust is invalidate the s1 to avoid the double free error.
    //
    // s1 -> Invalid
    //
    // s2 -> | name      | value |
    //       | ptr       | H1    |
    //       | len       | 5     |
    //       | capacity  | 5     |
    let s2 = s1;
}





// 4.
// Ways that variable and Data Interact: Clone
//
// If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
fn way_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}





// 5.
// Ownership and Functions
//
fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownerships(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                // x comes into scope

    makes_copy(x);     // x would move into the function
    // i32 is Copy, so its okay to still use x afterward
}

fn takes_ownerships(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens





// 6.
// Return Values and Scope
//
// Returning values can also transfer ownership.
fn return_values_and_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
//
// -----------------------------------------------------------------------------------------------
//
// It is possible to return multiple values using a tuple, as shown:
fn using_tuple_to_return() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}