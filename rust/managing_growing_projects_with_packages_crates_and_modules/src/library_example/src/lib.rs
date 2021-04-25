// Module tree
//
// crate
//   - front_of_house
//       - hosting
//           - add_to_waitlist
//           - seat_at_table
//       - serving
//           - take_order
//           - serve_order
//           - take_payment
//
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();
}

// Making Structs and Enums Public

mod back_of_house {
    pub enum Appetizer {
        Soup, // public
        Salad, // public
    }

    pub struct Breakfast {
        pub toast: String, // public
        seasonal_fruit: String, // private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with the Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind abour what bread we would like
    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);

    // The next line will not compile if we uncomment it; we are not allowed
    // to see or modify the seasonal_fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant3() {
    // Both are public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Bringing Paths into Scope with the use Keyword
//
pub use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
    add_to_waitlist();
    hosting::add_to_waitlist();
}

// Import more than 1 Result:
use std::fmt;
use std::io;

// Or you can provide new name -> 
//
// use std::fmt::Result
// use std::io::Result as IoResult

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }
