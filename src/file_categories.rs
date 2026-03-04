use std::path::PathBuf;

pub struct FileCategories {
    pub documents: Vec<PathBuf>,
    pub pictures: Vec<PathBuf>,
    pub videos: Vec<PathBuf>,
    pub music: Vec<PathBuf>,
    pub archives: Vec<PathBuf>,
    pub code_files: Vec<PathBuf>,
    pub others: Vec<PathBuf>,
}

impl FileCategories {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            pictures: Vec::new(),
            videos: Vec::new(),
            music: Vec::new(),
            archives: Vec::new(),
            code_files: Vec::new(),
            others: Vec::new(),
        }
    }
}
