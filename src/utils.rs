use colored::Colorize;
use std::{env, path::PathBuf};

pub fn get_organization_directory(path: &Option<String>) -> Option<PathBuf> {
    match path {
        Some(dir) => Some(PathBuf::from(dir)),
        None => match env::current_dir() {
            Ok(path) => Some(path),
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "ERROR:".red().bold(),
                    format!("Could not get current directory: {}", e)
                );
                None
            }
        },
    }
}

pub fn get_category(extension: &str) -> &str {
    match extension {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" => "pictures",
        "pdf" | "txt" | "doc" | "docx" | "odt" => "documents",
        "mp3" | "wav" | "flac" | "aac" => "music",
        "mp4" | "avi" | "mkv" | "mov" => "videos",
        "zip" | "rar" | "7z" | "tar" | "gz" => "archives",
        "rs" | "py" | "js" | "ts" | "html" | "css" | "jsx" | "tsx" | "json" | "sql" | "yml" => {
            "code_files"
        }
        _ => "others",
    }
}

pub fn separator() {
    println!("{}", "--------------------------------------------------------------------------------------------------------------".bright_black().bold());
}
