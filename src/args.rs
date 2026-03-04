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

    #[arg(long)]
    pub include_dirs: bool,

    #[arg(short, long, help = "Show detailed progress messages")]
    pub verbose: bool,
}
