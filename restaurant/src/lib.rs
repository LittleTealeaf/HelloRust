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

mod back_of_house {
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
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_at_restaurant_back() {

    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant_using() {
    hosting::add_to_waitlist();
}
// use the package instead of using the fuction and only importing that function

