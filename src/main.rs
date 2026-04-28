use rtd_fork::{add_item, list_all, list_completed, list_deleted, list_uncompleted};
fn main() {
    println!("RTD Clone - Rust Todo CLI");
    println!("Phase 6: optimize service logic.");
    println!();

    match add_item("learn Rust basic knowledge.") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }

    match add_item("finished phase 4 - service logic.") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }

    println!();
    
    println!("=== list uncompleted items === ");
    match list_uncompleted() {
        Ok(list) => println!("{}", list),
        Err(e) => println!("error: {}", e),
    }

    println!("=== list completed items ===");
    match list_completed() {
        Ok(list) => println!("{}", list),
        Err(e) => println!("error: {}", e),
    }

    println!("=== list deleted items ===");
    match list_deleted() {
        Ok(list) => println!("{}", list),
        Err(e) => println!("error: {}", e),
    }

    println!("=== list all items ===");
    match list_all() {
        Ok(list) => println!("{}", list),
        Err(e) => println!("error: {}", e),
    }

}