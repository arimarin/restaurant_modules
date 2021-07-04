mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { println!("Added to waitlist") }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    use std::fmt;

    pub enum Appetizer {
        Soup,
        Salad,
    }

    // Formatter to print Appetizer enum (copied from stackoverflow and modified)
    impl fmt::Display for Appetizer {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Appetizer::Soup => write!(f, "Soup"),
                Appetizer::Salad => write!(f, "Salad"),
            }
        }
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // call server_order
        super::serve_order();
    }

    fn cook_order() {}
}

// Abs path and pub use (re-export)
pub use crate::front_of_house::hosting;
// Relative path
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // With use
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("So second order was {1} and first {0}, right?", order1, order2);
}