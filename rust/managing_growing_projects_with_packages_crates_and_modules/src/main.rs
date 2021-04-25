fn main() {
    println!("Hello, world!");
}

//  ** You can find an implementation in src/library_example **

// 1. Packages and Crates
//
// Crates - Library or binary. The crate root is a source file that the Rust compiler starts from 
//  and makes up the root module of your crate.
//
// Packages - Is one or more crates that provide a set of functionality. A package contain Cargo.toml 
//  file that describes how to build those crates.

// 2. Defining Modules to Control Scope and Privacy
//
// Modules - Modules let us organize code within a crate into groups for readability and easy reuse.
//  modules also control the pricacy of items, which is whether an item can be used by outside code (public)
//  or is an internal implementation detail and not available for outside use (private)

// 3. Paths for Referring to an Item in the Module Tree
//
// To show Rust where to find an item in a module tree, we use a path in the same way we use a path when 
//  navigating a filesystem.
//
// A path take two forms:
//
//   * An absolute path starts from a crate root by using a crate name or a literal crate.
//
//   * A relative path starts from the current module and uses self, super, or an identifier in the current module.
//
// Both absolute and relative paths are followed by one or more identifiers separeted by double colons (::).