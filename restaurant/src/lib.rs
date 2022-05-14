// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use self::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }
mod front_of_house;
mod test_struct;

use String;
// pub use crate::front_of_house::hosting;
// use test_struct;

pub fn eat_at_restaurant() {
    front_of_house::aaa();
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    let a = test_struct::Breakfast {
        toast: String::from("hellp"),
        seasonal_fruit: String::from("hellp"),
    };
}
