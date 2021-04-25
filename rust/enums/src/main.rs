enum IpAddreKind {
    V4,
    V6,
}

fn main() {
    println!("Hello, world!");
}

// Instantiate enums
fn instantiate_enums() {
    let four = IpAddreKind::V4;
    let six = IpAddreKind::V6;
}

// Definig function with enums
fn defining_functions(ip_kind: IpAddreKind) {

}

// Defining as structure
struct IpAddr {
    kind: IpAddreKind,
    address: String,
}

fn defining_struct() {
    let home = IpAddr {
        kind: IpAddreKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddreKind::V6,
        address: String::from("::1")
    };
}

// Represent the struct over as enum

struct IpAddrV4 {
    // --snip--
}

struct IpAddrV6 {
    // --snip--
}

enum IpAddrrEnum {
    V4(IpAddrV4),
    V6(IpAddrV6),
}

fn represent_struct_as_enum() {
    let home = IpAddrrEnum::V4(IpAddrV4{});
    let loopback = IpAddrrEnum::V6(IpAddrV6{});
}

// Option values

fn option_values() {
    let some_number = Some(5);
    let some_string = Some("Some string");

    let absent_number: Option<i32> = None;
}

// You can not sum a T value with Option<T>, because they are different types. When we have an Option<T> we need to worry about possibly
// not having a value and the compiler will make sure we handle that case before using the value.
fn option_values_with_error() {
    let x: i8 = -5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // Error here
}

// The match Control Flow Operator

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// Matching with Option<T>

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// The _ Placeholder

fn the_underline_placeholder(value: u8) {
    match value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

// Concise Control Flow with if let

fn concise_control_flow_with_if_let() {
    let some_u8_value = Some(0u8);

    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    //
    // Is equal to:
    if let Some(3) = some_u8_value {
        println!("three")
    }
}