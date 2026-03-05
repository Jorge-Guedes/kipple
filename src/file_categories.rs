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

    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
            && self.pictures.is_empty()
            && self.videos.is_empty()
            && self.music.is_empty()
            && self.archives.is_empty()
            && self.code_files.is_empty()
            && self.others.is_empty()
    }
}
