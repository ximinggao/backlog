use clap::Parser;

mod enums;
mod structs;
mod api;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    first_name: String,
    #[arg(short, long)]
    last_name: String,
    #[arg(short, long, default_value_t = 1)]
    age: u8,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.first_name);
    println!("{:?}", args.last_name);
    println!("{:?}", args.age);
}
