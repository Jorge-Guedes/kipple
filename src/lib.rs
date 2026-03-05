pub mod args;
pub mod file_categories;
pub mod organizer;
pub mod utils;

pub use file_categories::FileCategories;
pub use organizer::{classify_file, organize_files, preview_organization};
pub use utils::{get_organization_directory, separator};
