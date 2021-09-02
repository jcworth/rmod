use std::{
    ffi::OsStr,
    fs::{self, ReadDir},
    io::{Error, Result},
    path::{Path, PathBuf},
};

use crate::utils;

// use crate::{error::RmError, spinner::Spinner, utils, NodeModuleMap};

// TODO: Refactor 'unwrap' calls w/ error handling

#[derive(Debug)]
pub struct Entry {
    abs_path: PathBuf,
    meta: fs::Metadata,
}

impl Entry {
    fn from_dir_entry(entry: fs::DirEntry) -> Result<Entry> {
        Ok(Entry {
            abs_path: entry.path(),
            meta: entry.metadata()?,
        })
    }

    pub fn meta(&self) -> &fs::Metadata {
        &self.meta
    }

    pub fn file_type(&self) -> fs::FileType {
        self.meta().file_type()
    }

    pub fn file_name(&self) -> &OsStr {
        self.abs_path.file_name().unwrap()
    }

    pub fn path(&self) -> &Path {
        self.abs_path.as_path()
    }

    pub fn is_node_modules(&self) -> bool {
        self.file_name() == OsStr::new("node_modules")
    }
}

#[derive(Debug)]
pub struct EntryIter {
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
                        // TODO: Options for node_modules, symlink, etc

                        // if node_modules return the entry and stop descending
                        if utils::is_node_modules(&entry.abs_path) {
                            // self.pop();
                            return Some(Ok(entry));
                        }

                        // If dir && not a symlink, descend and return folder
                        if entry.file_type().is_dir() && !entry.file_type().is_symlink() {
                            let read_dir = fs::read_dir(&entry.abs_path).unwrap();
                            self.push(read_dir);

                            // TODO: option to include folder
                            return Some(Ok(entry));
                        } else if !entry.file_type().is_symlink() {
                            return Some(Ok(entry));
                        }
                    }
                }
                Some(Err(e)) => return Some(Err(e)),
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct EntryWalk {
    list: Vec<ReadDir>,
}

impl EntryWalk {
    pub fn new(path: PathBuf) -> Result<Self> {
        let list = fs::read_dir(path).unwrap();
        Ok(EntryWalk { list: vec![list] })
    }
}

impl IntoIterator for EntryWalk {
    type Item = Result<Entry>;
    type IntoIter = EntryIter;

    fn into_iter(self) -> Self::IntoIter {
        EntryIter { list: self.list }
    }
}
