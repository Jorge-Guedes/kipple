use kipple::get_organization_directory;
use kipple::utils::{get_category, separator};

#[test]
fn test_get_category() {
    assert_eq!(get_category("jpg"), "pictures");
    assert_eq!(get_category("pdf"), "documents");
    assert_eq!(get_category("mp3"), "music");
    assert_eq!(get_category("mp4"), "videos");
    assert_eq!(get_category("zip"), "archives");
    assert_eq!(get_category("rs"), "code_files");
    assert_eq!(get_category("xyz"), "others");
}

#[test]
fn test_get_organization_directory_with_some() {
    let path = Some("/tmp".to_string());
    let result = get_organization_directory(&path);
    assert!(result.is_some());
    assert_eq!(result.unwrap().to_string_lossy(), "/tmp");
}

#[test]
fn test_get_organization_directory_with_none() {
    let result = get_organization_directory(&None);
    assert!(result.is_some());
}

#[test]
fn test_separator_does_not_panic() {
    separator();
}
