// use std::io::{self};
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {

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
fn way_move() {
    let x = 5; // bind the value 5 to x (pushed on to the stack)
    let y = x;  // copye the value of x(5) and bind to y (pushed on to the stack)


    // s1 -> | name      | value |              | index | value -> H1
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

// Ways that variable and Data Interact: Clone
