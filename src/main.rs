use rtd_fork::{Item, add_item, list_all, list_uncompleted};
fn main() {
    println!("RTD Clone - Rust Todo CLI");
    println!("Phase 4: file save && load.");
    println!();

    match add_item("learn Rust") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }

    match add_item("finished phase 4.") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    match list_uncompleted() {
        Ok(list) => println!("{}", list),
        Err(e) => println!("Error: {}", e),
    }

    println!();
    
    // ToString serilization
    println!("Direct serialization demo:");
    let item = Item::new(99, "test item");
    println!(" Item.to_string(): {}", item.to_string());
    println!(" Item ID: {}", item.id());
    println!(" Item name: {}", item.name());

    println!();

    match list_all() {
        Ok(list) => println!("{}", list),
        Err(e) => println!("Error, {}", e),
    }

}