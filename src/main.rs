use clap::{Parser};

mod service;
mod storage;
mod model;
use crate::service::{add_item, complete_item};

#[derive(Parser, Debug)]
struct Args {
    add: Option<String>,
    complete: Option<u32>,
}
fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    dbg!(&args);

    if let Some(name) = args.add {
        match add_item(&name) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("add '{name}' fail: {e}"),
        }
    }

    if let Some(id) = args.complete {
        match complete_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Complete tod '{id}' fail: {e}"),
        }
    }
}
