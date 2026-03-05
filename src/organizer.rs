use crate::file_categories::FileCategories;
use crate::utils::{get_category, separator};
use colored::Colorize;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn classify_file(dir: &PathBuf, include_dirs: bool, verbose: bool) -> FileCategories {
    let mut categories = FileCategories::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if include_dirs && path.is_dir() {
                    if verbose {
                        println!("{} 📁 {}", "DIR:".blue().bold(), path.display());
                        separator();
                    }
                    process_dir(&path, &mut categories, verbose);
                } else if !include_dirs && path.is_dir() && verbose {
                    println!(
                        "{} {}",
                        "WARNING:".yellow().bold(),
                        format!("Directory skipped: {:?} (use --include-dirs)", path)
                    );
                    separator();
                } else if !path.is_dir() {
                    process_file(&path, &mut categories, verbose);
                }
            }
        }
    }

    categories
}

fn process_dir(dir: &Path, categories: &mut FileCategories, verbose: bool) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    process_dir(&path, categories, verbose);
                } else {
                    process_file(&path, categories, verbose);
                }
            }
        }
    }
}

fn process_file(path: &PathBuf, categories: &mut FileCategories, verbose: bool) {
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
                    separator();
                }
            }

            match category {
                "pictures" => categories.pictures.push(path.clone()),
                "documents" => categories.documents.push(path.clone()),
                "music" => categories.music.push(path.clone()),
                "videos" => categories.videos.push(path.clone()),
                "archives" => categories.archives.push(path.clone()),
                "code_files" => categories.code_files.push(path.clone()),
                _ => categories.others.push(path.clone()),
            }
        } else {
            if verbose {
                println!(
                    "{} {}",
                    "WARNING:".yellow().bold(),
                    format!("File with non-UTF8 extension: {:?}", path)
                );
                separator();
            }
            categories.others.push(path.clone());
        }
    } else {
        if verbose {
            println!(
                "{} {}",
                "WARNING:".yellow().bold(),
                format!("File with no extension: {:?}", path)
            );
            separator();
        }
        categories.others.push(path.clone());
    }
}

pub fn organize_files(files: &FileCategories, dir: &PathBuf, force: bool, verbose: bool) {
    let categories: [(&str, &Vec<PathBuf>); 7] = [
        ("Pictures", &files.pictures),
        ("Documents", &files.documents),
        ("Music", &files.music),
        ("Videos", &files.videos),
        ("Archives", &files.archives),
        ("Code", &files.code_files),
        ("Others", &files.others),
    ];

    for (title, items) in &categories {
        if !items.is_empty() {
            let category_dir = dir.join(title);
            if let Err(e) = fs::create_dir_all(&category_dir) {
                eprintln!(
                    "{} {}",
                    "ERROR:".red().bold(),
                    format!("Creating directory {:?}: {}", category_dir, e)
                );
                continue;
            }

            for item in *items {
                if let Some(name) = item.file_name() {
                    let destination = category_dir.join(name);
                    let final_destination = if force {
                        if verbose {
                            println!(
                                "{} {} {}",
                                "FORCE:".red().bold(),
                                "Overwriting".white(),
                                destination.display()
                            );
                        }
                        destination
                    } else {
                        get_unique_destination(&destination, verbose)
                    };
                    if let Err(e) = std::fs::rename(item, &final_destination) {
                        eprintln!(
                            "{} {}",
                            "ERROR:".red().bold(),
                            format!("Moving {:?}: {}", item, e)
                        );
                    } else {
                        if verbose {
                            println!(
                                "{} {} {} {}",
                                "MOVED:".green().bold(),
                                final_destination
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .cyan(),
                                "→".magenta().bold(),
                                format!("{}", category_dir.display()).blue()
                            );
                            separator();
                        }
                    }
                }
            }
        }
    }

    remove_empty_dirs(dir, verbose);
}

fn get_unique_destination(destination: &PathBuf, verbose: bool) -> PathBuf {
    if !destination.exists() {
        return destination.clone();
    }

    if verbose {
        println!(
            "{} {} {}",
            "DUPLICATE:".yellow().bold(),
            destination
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .cyan(),
            "already exists, looking for alternative...".white()
        );
        separator();
    }

    let stem = destination
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let extension = destination
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let mut counter = 1;
    loop {
        let new_name = format!("{}({}).{}", stem, counter, extension);
        let new_destination = destination.with_file_name(new_name);

        if !new_destination.exists() {
            if verbose {
                println!(
                    "{} {} {} {}",
                    "RENAMED:".yellow().bold(),
                    destination
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .cyan(),
                    "→".magenta().bold(),
                    new_destination
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .cyan()
                );
                separator();
            }
            return new_destination;
        }

        counter += 1;
    }
}

fn remove_empty_dirs(dir: &Path, verbose: bool) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    remove_empty_dirs(&path, verbose);

                    if let Ok(mut read_dir) = fs::read_dir(&path) {
                        if read_dir.next().is_none() {
                            if let Err(e) = fs::remove_dir(&path) {
                                eprintln!(
                                    "{} {}",
                                    "ERROR:".red().bold(),
                                    format!("Removing directory {:?}: {}", path, e)
                                );
                            } else if verbose {
                                println!("{} {}", "REMOVED:".red().bold(), path.display());
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn preview_organization(files: &FileCategories) {
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
