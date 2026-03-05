mod args;
mod file_categories;
mod organizer;
mod utils;

use args::Args;
use clap::Parser;
use colored::Colorize;
use organizer::{classify_file, organize_files, preview_organization};
use utils::{get_organization_directory, separator};

fn main() {
    let args = Args::parse();

    let Some(selected_dir) = get_organization_directory(&args.directory) else {
        eprintln!(
            "{} {}",
            "ERROR:".red().bold(),
            "Could not determine target directory"
        );
        return;
    };

    if args.verbose {
        println!(
            "{} {}",
            "PATH SELECTED:".green().bold(),
            selected_dir.display().to_string().magenta()
        );
        separator();
    }

    let classified_files = classify_file(&selected_dir, args.include_dirs, args.verbose);

    if classified_files.is_empty() {
        println!(
            "{} {}",
            "INFO:".yellow().bold(),
            format!("No files to organize in {}", selected_dir.display())
        );
        return;
    }

    if args.dry_run {
        preview_organization(&classified_files);
    } else {
        organize_files(&classified_files, &selected_dir, args.verbose);
        println!(
            "{} {} {}",
            "SUCCESS:".green().bold(),
            "Files organized in".white(),
            selected_dir.display().to_string().blue()
        );
    }
}
