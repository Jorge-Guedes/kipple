mod args;
use args::Args;
mod file_categories;
use clap::Parser;
use file_categories::FileCategories;
use std::{env, fs, path::PathBuf};

fn main() {
    let args = Args::parse();

    let Some(selected_dir) = get_organization_directory(&args.directory) else {
        eprintln!("Error: Could not determine target directory");
        return;
    };

    if args.verbose {
        println!("Ruta seleccionada: {}", selected_dir.display())
    }

    let classified_files = classify_file(&selected_dir, args.include_dirs, args.verbose);
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
                    println!("{}", path.display())
                } else {
                    if !path.is_dir() {
                        if let Some(extension) = path.extension() {
                            if let Some(ext_str) = extension.to_str() {
                                if verbose {
                                    println!("Evaluating extension: .{}", ext_str);
                                }
                                let category = get_category(ext_str);
                                if verbose {
                                    println!("Classified as: {}", category);
                                }

                                match category {
                                    "images" => categories.pictures.push(path),
                                    "documents" => categories.documents.push(path),
                                    "music" => categories.music.push(path),
                                    "videos" => categories.videos.push(path),
                                    "archives" => categories.archives.push(path),
                                    "code_files" => categories.code_files.push(path),
                                    _ => categories.others.push(path),
                                }
                            } else {
                                if verbose {
                                    println!("Skipping file with non-UTF8 extension: {:?}", path)
                                }
                            }
                        } else {
                            if verbose {
                                println!("Skipping file with no extension: {:?}", path)
                            }
                        }
                    } else {
                        if verbose {
                            println!(
                                "Skipping directory: {:?} (use --include-dirs to include)",
                                path
                            )
                        }
                    }
                }
            }
        }
    }

    categories
}
