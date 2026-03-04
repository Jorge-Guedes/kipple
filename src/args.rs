use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Jorge Guedes",
    version = "1.0.0",
    about = "Kipple - File organizer"
)]
pub struct Args {
    #[arg(short, long)]
    pub directory: Option<String>,

    #[arg(long)]
    dry_run: bool,

    #[arg(short, long)]
    include_dirs: bool,
}
