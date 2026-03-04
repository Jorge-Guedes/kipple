mod args;
use args::Args;
use clap::Parser;
use std::{env, path::PathBuf};

fn main() {
    let args = Args::parse();

    let Some(selected_dir) = get_organization_directory(&args.directory) else {
        eprintln!("Error: Could not determine target directory");
        return;
    };

    println!("Ruta seleccionada: {}", selected_dir.display())
}

fn get_organization_directory(path: &Option<String>) -> Option<PathBuf> {
    match path {
        Some(dir) => Some(PathBuf::from(dir)),
        None => match env::current_dir() {
            Ok(path) => Some(path),
            Err(e) => {
                eprintln!("Error getting current directory: {}", e);
                None
            }
        },
    }
}
