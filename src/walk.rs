use std::{fs::{self, ReadDir}, io::Result, path::PathBuf};

// use crate::{error::RmError, spinner::Spinner, utils, NodeModuleMap};

// TODO: Refactor 'unwrap' calls w/ error handling

#[derive(Debug)]
struct Entry {
    abs_path: PathBuf,
    file_type: fs::FileType,
    size: u64
}

impl Entry {
    fn from_dir_entry(entry: fs::DirEntry) -> Result<Entry> {
        Ok(Entry {
            abs_path: entry.path(),
            file_type: entry.file_type()?,
            size: entry.metadata()?.len()
        })
    }
}

#[derive(Debug)]
struct EntryIter {
    list: Vec<ReadDir>,
}

impl EntryIter {
    fn pop(&mut self) {
        self.list.pop();
    }

    fn push(&mut self, v: ReadDir) {
        self.list.push(v);
    }
}

impl Iterator for EntryIter {
    type Item = Result<Entry>;

    fn next(&mut self) -> Option<Result<Entry>> {
        while !self.list.is_empty() {
            let next_item = self.list.last_mut().unwrap().next();

            match next_item {
                None => self.pop(),
                Some(Ok(entry)) => {
                    if let Ok(entry) = Entry::from_dir_entry(entry) {
                        if entry.file_type.is_dir() && !entry.file_type.is_symlink() {
                            let read_dir = fs::read_dir(&entry.abs_path).unwrap();
                            self.push(read_dir);

                            // TODO: option to include folder
                            return Some(Ok(entry));
                        } else if !entry.file_type.is_symlink() {
                            return Some(Ok(entry));
                        }
                    }
                },
                Some(Err(e)) => {
                    return Some(Err(e))
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct EntryWalk {
    list: Vec<ReadDir>,
}

impl EntryWalk {
    fn new(path: PathBuf) -> Self {
        let list = fs::read_dir(path).unwrap();
        EntryWalk { list: vec![list] }
    }
}

impl IntoIterator for EntryWalk {
    type Item = Result<Entry>;
    type IntoIter = EntryIter;

    fn into_iter(self) -> Self::IntoIter {
        EntryIter {
            list: self.list,
        }
    }
}
