mod args;
use args::Args;
mod file_categories;
use clap::Parser;
use colored::Colorize;
use file_categories::FileCategories;
use std::{env, fs, path::PathBuf};

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
        println!("{} {}", "SELECTED:".green().bold(), selected_dir.display());
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
        organize_files(&classified_files, &selected_dir);
    }
}

fn get_organization_directory(path: &Option<String>) -> Option<PathBuf> {
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

fn get_category(extension: &str) -> &str {
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

fn classify_file(dir: &PathBuf, include_dirs: bool, verbose: bool) -> FileCategories {
    let mut categories = FileCategories::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if include_dirs {
                    if verbose {
                        if let Some(name) = path.file_name() {
                            println!(
                                "{} 📁 {} {}",
                                "DIR:".blue().bold(),
                                "→".magenta().bold(),
                                name.to_string_lossy(),
                            );
                        }
                        println!("{}", "--------------------------------------------------------------------------------------------------------------".bright_black().bold());
                    }
                } else {
                    if !path.is_dir() {
                        if let Some(extension) = path.extension() {
                            if let Some(ext_str) = extension.to_str() {
                                if verbose {
                                    if let Some(name) = path.file_name() {
                                        println!(
                                            "{} {} {}{}{}",
                                            "EVALUATING:".yellow().bold(),
                                            name.to_string_lossy().white(),
                                            "(".yellow().bold(),
                                            format!(".{}", ext_str).cyan(),
                                            ")".yellow().bold()
                                        );
                                    }
                                }
                                let category = get_category(ext_str);
                                if verbose {
                                    if let Some(name) = path.file_name() {
                                        println!(
                                            "{} {} {} 📁 {}",
                                            "CLASSIFIED:".green().bold(),
                                            name.to_string_lossy().white(),
                                            "→".magenta().bold(),
                                            category.to_string().magenta().bold()
                                        );
                                        println!("{}", "--------------------------------------------------------------------------------------------------------------".bright_black().bold());
                                    }
                                }

                                match category {
                                    "pictures" => categories.pictures.push(path),
                                    "documents" => categories.documents.push(path),
                                    "music" => categories.music.push(path),
                                    "videos" => categories.videos.push(path),
                                    "archives" => categories.archives.push(path),
                                    "code_files" => categories.code_files.push(path),
                                    _ => categories.others.push(path),
                                }
                            } else {
                                if verbose {
                                    println!(
                                        "{} {}",
                                        "WARNING:".yellow().bold(),
                                        format!("File with non-UTF8 extension: {:?}", path)
                                    );
                                    println!("{}", "--------------------------------------------------------------------------------------------------------------".bright_black().bold());
                                }
                            }
                        } else {
                            if verbose {
                                println!(
                                    "{} {}",
                                    "WARNING:".yellow().bold(),
                                    format!("File with no extension: {:?}", path)
                                );
                                println!("{}", "--------------------------------------------------------------------------------------------------------------".bright_black().bold());
                            }
                        }
                    } else {
                        if verbose {
                            println!(
                                "{} {}",
                                "WARNING:".yellow().bold(),
                                format!(
                                    "Directory skipped: {:?} (use --include-dirs to include)",
                                    path
                                )
                            );
                            println!("{}", "--------------------------------------------------------------------------------------------------------------".bright_black().bold());
                        }
                    }
                }
            }
        }
    }

    categories
}

fn preview_organization(files: &FileCategories) {
    let categories: [(&str, &Vec<PathBuf>); 7] = [
        ("📁 Pictures", &files.pictures),
        ("📁 Documents", &files.documents),
        ("📁 Music", &files.music),
        ("📁 Videos", &files.videos),
        ("📁 Archives", &files.archives),
        ("📁 Code", &files.code_files),
        ("📁 Others", &files.others),
    ];

    for i in 0..categories.len() {
        let (title, items) = &categories[i];
        if !items.is_empty() {
            println!(
                "{} {}",
                title.magenta().bold(),
                format!("({})", items.len()).yellow()
            );
            for file in *items {
                if let Some(name) = file.file_name() {
                    println!(
                        "   {}",
                        format!("{} {}", "-".magenta().bold(), name.to_string_lossy()).cyan()
                    );
                }
            }
        }
    }
}

fn organize_files(files: &FileCategories, dir: &PathBuf) {
    todo!()
}
