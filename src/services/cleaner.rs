use std::{fs, io};
use std::path::Path;
use crate::services::logging::log_message;

pub fn clean_folder(path: &Path) -> io::Result<()> {
    log_message(&format!("Cleaning folder: {}", path.display()));

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Folder not found: {}", path.display())));
    }

    if !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Not a directory: {}", path.display())));
    }

    if is_dangerous(path) {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Refusing to clean root directory"));
    }

    let mut files_removed = 0usize;
    let mut dirs_removed = 0usize;
    let mut errors = 0usize;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();

        let result = if p.is_dir() { fs::remove_dir_all(&p) } else { fs::remove_file(&p) };

        match result {
            Ok(_) => {
                if p.is_dir() {
                    dirs_removed += 1;
                } else {
                    files_removed += 1;
                }
            }
            Err(_) => {
                errors += 1;
            }
        }
    }

    log_message(&format!(
        "Cleaned: {} | files={} dirs={} errors={}",
        path.display(),
        files_removed,
        dirs_removed,
        errors
    ));

    Ok(())
}

fn is_dangerous(path: &Path) -> bool {
    path.parent().is_none() // C:\ или /
}