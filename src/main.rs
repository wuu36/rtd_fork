use std::arch::x86_64::_MM_MANT_SIGN_ZERO;

use rtd_fork::{Item, add_item, list_uncompleted};
fn main() {
    println!("RTD Clone - Rust Todo CLI");
    println!();

    match add_item("learn Rust") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }

    match add_item("finished phase 3.") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    match list_uncompleted() {
        Ok(list) => println!("{}", list),
        Err(e) => println!("Error: {}", e),
    }

    let item = Item::new(99, "test item");
    println!();
    println!("direct item: [{}] {}", item.id(), item.name());
}