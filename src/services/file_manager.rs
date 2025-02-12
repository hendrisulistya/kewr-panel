use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use crate::config::WorkingDirectoryConfig;
use serde::Serialize;

pub struct FileManagerService {
    working_dirs: WorkingDirectoryConfig,
}

#[derive(Debug, Serialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: String,
}

impl FileManagerService {
    pub fn new(working_dirs: WorkingDirectoryConfig) -> Self {
        Self { working_dirs }
    }

    pub fn list_directory(&self, relative_path: &str) -> io::Result<Vec<FileInfo>> {
        let base_path = self.working_dirs.base_dir.join(relative_path);
        if !self.is_path_allowed(&base_path) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied"));
        }

        let entries = fs::read_dir(base_path)?;
        let mut files = Vec::new();

        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let modified = metadata.modified()?
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            files.push(FileInfo {
                name: entry.file_name().to_string_lossy().into_owned(),
                path: entry.path().to_string_lossy().into_owned(),
                is_dir: metadata.is_dir(),
                size: metadata.len(),
                modified: modified.to_string(),
            });
        }

        Ok(files)
    }

    pub fn read_file(&self, relative_path: &str) -> io::Result<String> {
        let file_path = self.working_dirs.base_dir.join(relative_path);
        if !self.is_path_allowed(&file_path) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied"));
        }

        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn write_file(&self, relative_path: &str, content: &str) -> io::Result<()> {
        let file_path = self.working_dirs.base_dir.join(relative_path);
        if !self.is_path_allowed(&file_path) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied"));
        }

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn delete_file(&self, relative_path: &str) -> io::Result<()> {
        let file_path = self.working_dirs.base_dir.join(relative_path);
        if !self.is_path_allowed(&file_path) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied"));
        }

        let metadata = fs::metadata(&file_path)?;
        if metadata.is_dir() {
            fs::remove_dir_all(file_path)
        } else {
            fs::remove_file(file_path)
        }
    }

    fn is_path_allowed(&self, path: &Path) -> bool {
        path.starts_with(&self.working_dirs.base_dir) ||
        path.starts_with(&self.working_dirs.www_dir) ||
        path.starts_with(&self.working_dirs.logs_dir) ||
        path.starts_with(&self.working_dirs.config_dir) ||
        path.starts_with(&self.working_dirs.run_dir)
    }
}