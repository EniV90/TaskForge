mod api;
mod enums;
mod structs;

use structs::done::Done;
use structs::pending::Pending;

use crate::enums::TaskStatus;
use api::basic_actions::create::create;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    status: String,
}
fn main() -> Result<(), String> {
    let done = Done::new("shopping");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);

    let pending = Pending::new("Eat");
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status);

    let args = Args::parse();
    let status_enum = TaskStatus::from_string(&args.status)?;
    let to_do_item = create(&args.title, status_enum)?;
    println!("{}", to_do_item);
    Ok(())
}

// mod api;
// mod enums;
// mod structs;

// use structs::done::Done;
// use structs::pending::Pending;

// use crate::enums::TaskStatus;
// use api::basic_actions::create::create;
// use clap::Parser;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     #[arg(short, long)]
//     title: String,
//     #[arg(short, long)]
//     status: String,
// }

// fn main() -> Result<(), String> {
//     let args = Args::parse();
//     let status_enum = TaskStatus::from_string(&args.status)?;
//     let to_do_item = create(&args.title, status_enum)?;
//     println!("{}", to_do_item);
//     Ok(())
// }
