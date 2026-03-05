use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Jorge Guedes",
    version = "1.0.0",
    about = "Kipple - File organizer",
    long_about = "Kipple organizes files by extension into categorized folders.\n\n\
              Examples:\n  \
              kipple                     # organize current directory\n  \
              kipple -d ~/Downloads       # organize Downloads folder\n  \
              kipple --dry-run -v         # preview with details\n  \
              kipple --include-dirs       # also process subdirectories\n  \
              kipple --force              # overwrite existing files"
)]
pub struct Args {
    #[arg(short, long, help = "Target directory (defaults to current directory)")]
    pub directory: Option<String>,

    #[arg(long, help = "Preview changes without moving any files")]
    pub dry_run: bool,

    #[arg(
        long,
        help = "Process directories recursively: extract all files from subfolders, move them to their corresponding category folders (Images, Documents, etc.), and remove the original subfolders when empty"
    )]
    pub include_dirs: bool,

    #[arg(
        long,
        help = "Overwrite existing files instead of creating numbered copies (e.g., file(1).txt)"
    )]
    pub force: bool,

    #[arg(
        short,
        long,
        help = "Display detailed progress messages during execution"
    )]
    pub verbose: bool,
}
