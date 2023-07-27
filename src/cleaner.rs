use crate::folder::Folder;
use byte_unit::Byte;
use std::{io::Error as IOError, path::Path};
use strum::IntoEnumIterator;

pub struct Cleaner {
    space_cleaned: u64,
    pretend: bool,
}

impl Cleaner {
    pub const fn new(pretend: bool) -> Self {
        Self {
            space_cleaned: 0,
            pretend,
        }
    }

    pub fn perform(&mut self) {
        for folder in Folder::iter() {
            if folder.exists() {
                log::info!("Cleaning {}...", folder.pretty_name());
                self.clean(&folder).unwrap();
            }
        }

        let space = Byte::from(self.space_cleaned);
        log::info!("Total space cleaned: {}", space.get_appropriate_unit(false));
    }

    fn clean(&mut self, folder: &Folder) -> Result<(), IOError> {
        for entry in std::fs::read_dir(folder.folder_path())?
            .flatten()
            .filter(|e| !is_hidden(e))
            .filter(|e| folder.is_cache_file(&e.path()))
        {
            let path = entry.path();
            self.space_cleaned += get_size(&path)?;

            if !self.pretend {
                if entry.file_type()?.is_dir() {
                    std::fs::remove_dir_all(&path)?;
                } else {
                    std::fs::remove_file(&path)?;
                }
            }
            log::debug!("{} has been deleted", path.display());
        }

        Ok(())
    }
}

fn get_size(path: &Path) -> Result<u64, IOError> {
    let Ok(metadata) = std::fs::metadata(path) else {
        return Ok(0);
    };

    let mut size = metadata.len();

    if metadata.is_dir() {
        for entry in std::fs::read_dir(path)? {
            size += get_size(&entry?.path())?;
        }
    }
    Ok(size)
}

fn is_hidden(entry: &std::fs::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'))
}
