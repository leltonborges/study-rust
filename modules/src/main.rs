// use crate::house::{bedroom1, bedroom2};
mod bedroom;

use bedroom::{bedroom1, bedroom2};

fn main() {
    println!("Hello, world!");
    println!("bedroom1: {}", bedroom1::is_light_on());
    println!("bedroom1: {}", bedroom1::is_neighbour_light_on());
    println!("bedroom2: {}", bedroom2::is_light_on());
    println!("{}", bedroom::HOUSE_NUMBER);
}
