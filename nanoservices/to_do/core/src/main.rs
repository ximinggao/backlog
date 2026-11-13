use clap::Parser;

use crate::api::basic_actions::create::create;
use to_do_dal::to_do_items::{
    descriptors::JsonFileDescriptor, enums::TaskStatus, schema::NewToDoItem,
};

mod api;
mod enums;
mod structs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    status: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let status_enum = TaskStatus::from_string(&args.status)?;
    let new_to_do_item = NewToDoItem {
        title: args.title,
        status: status_enum,
    };

    let item = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let to_do_item = create::<JsonFileDescriptor>(new_to_do_item).await?;
            Ok(to_do_item)
        })?;

    println!("{:?}", item);
    Ok(())
}
