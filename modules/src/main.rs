use std::collections::HashMap;
pub use crate::front_of_house::hosting;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}