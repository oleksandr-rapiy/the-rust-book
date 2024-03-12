// NOTE: here we declare the module, which refer to the file with same name
mod front_of_house;
mod user;

// NOTE: we should have the same folder name as parent
use user::UserEntity;

pub fn eat_at_restaurant() {
    // NOTE: absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // NOTE: relative path
    front_of_house::hosting::add_to_wait_list();

    UserEntity::default();
}

fn serve_order() {}

mod back_of_house {

    #[derive(Debug)]
    pub struct Order {
        name: String,
        price: f64,
        desc: String,
        status: OrderStatus,
    }

    #[derive(Debug)]
    enum OrderStatus {
        Pending,
        Failed,
        Completed,
    }

    impl Order {
        fn create_order(name: &str) -> Order {
            Order {
                name: String::from(name),
                price: 12.2,
                desc: String::from(format!("{} - food is testy.", name)),
                status: OrderStatus::Pending,
            }
        }
        fn cook_order(&mut self) {
            self.status = OrderStatus::Completed;
        }
    }

    fn fix_incorrect_order() {
        let mut order = Order::create_order("Some name");
        cook_order(&mut order);

        // NOTE: super allows to reference the parent module
        super::serve_order()
    }

    fn cook_order(order: &mut Order) {
        order.cook_order();
    }
}

mod back_of_house_v2 {
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

        pub fn default() -> Breakfast {
            Breakfast {
                toast: String::new(),
                seasonal_fruit: String::new(),
            }
        }
    }
}

pub fn eat_at_restaurant_v2() {
    let mut meal = back_of_house_v2::Breakfast::summer("Rye");

    meal.toast = String::from("Another");

    let meal2 = back_of_house_v2::Breakfast::default();
}

use custom_result::Custom_Result;
use custom_result_v2::Custom_Result as CustomResultV2;

// NOTE: we add 'pub' to provide external code ability to use add_to_wait_list()
pub use crate::front_of_house_v2::hosting::add_to_wait_list;

mod front_of_house_v2 {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }
}

pVub fn eat_at_restaurant_v3() {
    add_to_wait_list();
}

pub mod custom_result {
    pub struct Custom_Result {}
}

pub mod custom_result_v2 {
    pub struct Custom_Result {}
}

fn get_custom_result() -> Custom_Result {
    Custom_Result {}
}

fn get_custom_result_v2() -> CustomResultV2 {
    CustomResultV2 {}
}

use rand::{Rng, RngCore};

// NOTE: we use self because
// use std::io;
// use std::io::Write;

use std::io::{self, Write};

// NOTE: glob operator
use std::io::*;

pub fn eat_at_restaurant_v4() {
    let rand_num = rand::thread_rng().gen_range(1..10);
}
