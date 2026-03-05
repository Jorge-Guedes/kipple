use kipple::FileCategories;
use kipple::{classify_file, organize_files, preview_organization};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_classify_file_basic() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path().to_path_buf();

    fs::write(test_dir.join("foto.jpg"), "contenido").unwrap();
    fs::write(test_dir.join("documento.pdf"), "contenido").unwrap();
    fs::write(test_dir.join("cancion.mp3"), "contenido").unwrap();

    let categories = classify_file(&test_dir, false, false);

    assert_eq!(categories.pictures.len(), 1);
    assert_eq!(categories.documents.len(), 1);
    assert_eq!(categories.music.len(), 1);
    assert_eq!(categories.videos.len(), 0);
}

#[test]
fn test_classify_file_with_unknown_extensions() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path().to_path_buf();

    fs::write(test_dir.join("archivo.xyz"), "contenido").unwrap();
    fs::write(test_dir.join("sin_extension"), "contenido").unwrap();

    let categories = classify_file(&test_dir, false, false);

    assert_eq!(categories.others.len(), 2);
}

#[test]
fn test_organize_files_moves_files() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path().to_path_buf();

    fs::write(test_dir.join("foto.jpg"), "contenido").unwrap();
    let mut categories = FileCategories::new();
    categories.pictures.push(test_dir.join("foto.jpg").clone());

    organize_files(&categories, &test_dir, false, false);

    assert!(test_dir.join("Pictures").exists());
    assert!(test_dir.join("Pictures").join("foto.jpg").exists());
    assert!(!test_dir.join("foto.jpg").exists());
}

#[test]
fn test_organize_files_with_duplicates() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path().to_path_buf();

    fs::write(test_dir.join("foto.jpg"), "original").unwrap();
    let mut categories = FileCategories::new();
    categories.pictures.push(test_dir.join("foto.jpg").clone());

    organize_files(&categories, &test_dir, false, false);

    fs::write(test_dir.join("foto.jpg"), "nuevo").unwrap();
    let mut categories2 = FileCategories::new();
    categories2.pictures.push(test_dir.join("foto.jpg").clone());

    organize_files(&categories2, &test_dir, false, false);

    assert!(test_dir.join("Pictures").join("foto.jpg").exists());
    assert!(test_dir.join("Pictures").join("foto(1).jpg").exists());
}

#[test]
fn test_organize_files_with_force() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path().to_path_buf();

    fs::write(test_dir.join("foto.jpg"), "original").unwrap();
    let mut categories = FileCategories::new();
    categories.pictures.push(test_dir.join("foto.jpg").clone());

    organize_files(&categories, &test_dir, false, false);

    fs::write(test_dir.join("foto.jpg"), "nuevo").unwrap();
    let mut categories2 = FileCategories::new();
    categories2.pictures.push(test_dir.join("foto.jpg").clone());

    organize_files(&categories2, &test_dir, true, false);

    assert!(test_dir.join("Pictures").join("foto.jpg").exists());
    assert!(!test_dir.join("Pictures").join("foto(1).jpg").exists());

    let content = fs::read_to_string(test_dir.join("Pictures").join("foto.jpg")).unwrap();
    assert_eq!(content, "nuevo");
}

#[test]
fn test_include_dirs_recursive() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path().to_path_buf();

    fs::create_dir(test_dir.join("subcarpeta")).unwrap();
    fs::write(test_dir.join("subcarpeta").join("foto.jpg"), "contenido").unwrap();

    let categories = classify_file(&test_dir, true, false);
    assert_eq!(categories.pictures.len(), 1);
    assert!(categories.pictures[0].ends_with("foto.jpg"));
}

#[test]
fn test_empty_dir_removal() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path().to_path_buf();

    fs::create_dir(test_dir.join("subcarpeta")).unwrap();
    fs::write(test_dir.join("subcarpeta").join("foto.jpg"), "contenido").unwrap();

    let categories = classify_file(&test_dir, true, false);
    organize_files(&categories, &test_dir, false, false);

    assert!(!test_dir.join("subcarpeta").exists());
}

#[test]
fn test_preview_organization_does_not_move_files() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path().to_path_buf();

    fs::write(test_dir.join("foto.jpg"), "contenido").unwrap();
    let categories = classify_file(&test_dir, false, false);

    preview_organization(&categories);

    assert!(test_dir.join("foto.jpg").exists());
    assert!(!test_dir.join("Pictures").exists());
}
