mod back_of_house;
mod front_of_house;
use front_of_house::hosting;
// pub use crate::front_of_house::hosting;


fn make_order() {}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();  // Absolute path
    front_of_house::hosting::add_to_waitlist();  // Relative path
    crate::hosting::add_to_waitlist();  // Re-export path (pub mod hosting)
    hosting::seat_at_table();  // Relative export path

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
}


// Новый модуль прямо в файле
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() { println!("add_to_waitlist"); }
//
//         pub fn seat_at_table() { println!("seat_at_table"); }
//     }
//
//     mod serving {
//         fn take_order() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }

// Новый модуль прямо в файле
mod customer {
    pub mod human {
        pub fn eat_at_restaurant() { println!("Human eat in restaurant"); }
        pub fn make_order() {}
    }
    mod private_db {
        pub fn save_changes() {}
    }

    pub struct Human {
        pub name: String,
        id: u64
    }
    impl Human {
        pub fn make_order() {}
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}






/// ## Rust 11 (DocTest)
/// Обычно документационные комментарии могут содержать секции "Examples", "Panics" and "Failures".
///
/// Следующая функция делит два числа.
///
/// # Examples
///
/// ```
/// let result = rust_7_restaurant::squared_number(5);
/// assert_eq!(result, 25);
/// ```
///
/// # Panics
///
/// Функция паникует, если второй аргумент равен нулю.
///
/// ```rust,should_panic
/// // паникует ...
/// rust_7_restaurant::squared_number(5);
/// assert!(false)
/// ```
pub fn squared_number(x: u32) -> u32 {
    x * x
}
