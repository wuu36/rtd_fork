use clap::{Parser, ValueEnum};
use rtd_fork::{add_item, list_all, list_completed, list_deleted, list_uncompleted};

#[derive(ValueEnum, Clone, Debug)]
enum ListType {
    All,
    Completed,
    Uncompleted,
    Deleted,
}

#[derive(Parser)]
#[command(name = "rtd")]
#[command(version = "0.1.0")]
#[command(about = "A simple todo CLI tool written in Rust", long_about = None)]
struct Args {
    #[arg(short, long, value_name = "NAME")]
    add: Option<String>,

    #[arg(short, long, value_name = "TYPE")]
    list: Option<Option<ListType>>,
}

fn main() {
    println!("RTD Clone - Rust Todo CLI");

    let args = Args::parse();

    if let Some(name) = args.add {
        match add_item(&name) {
            Ok(msg) => println!("{}", msg),
            Err(e) => eprintln!("error: {}", e),
        }
        return;
    }

    // 处理 list 命令 (默认列出未完成)
    let list_type = args.list.unwrap_or(Some(ListType::Uncompleted));
    
    match list_type {
        Some(ListType::All) => match list_all() {
            Ok(list) => println!("{}", list),
            Err(e) => eprintln!("Error: {}", e),
        },
        Some(ListType::Completed) => match list_completed() {
            Ok(list) => println!("{}", list),
            Err(e) => eprintln!("Error: {}", e),
        },
        Some(ListType::Uncompleted) | None => match list_uncompleted() {
            Ok(list) => println!("{}", list),
            Err(e) => eprintln!("Error: {}", e),
        },
        Some(ListType::Deleted) => match list_deleted() {
            Ok(list) => println!("{}", list),
            Err(e) => eprintln!("Error: {}", e),
        },
    }

}