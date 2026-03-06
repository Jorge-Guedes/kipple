use clap::Parser;
use colored::Colorize;
use kipple::args::Args;
use kipple::config::get_config_path;
use kipple::organizer::{classify_file, organize_files, preview_organization};
use kipple::utils::{get_organization_directory, separator};

fn main() {
    let args = Args::parse();

    if true {
        // Modo pruebas - SOLO para probar el config
        let config_path = match &args.config {
            Some(path) => match get_config_path(path) {
                Some(p) => p,
                None => {
                    println!("ERROR: Config file not found");
                    return;
                }
            },
            None => {
                println!("No config file specified");
                return;
            }
        };
        println!("RUTA: {:?}", config_path);
        return;
    }

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
        organize_files(&classified_files, &selected_dir, args.force, args.verbose);
        println!(
            "{} {} {}",
            "SUCCESS:".green().bold(),
            "Files organized in".white(),
            selected_dir.display().to_string().blue()
        );
    }
}
