use std::{
    ffi::OsStr,
    fs::{self, ReadDir},
    io::Result,
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
    options: EntryOptions,
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
            let next_item = self.list.last_mut()?.next();

            match next_item {
                None => self.pop(),
                Some(Ok(entry)) => {
                    if let Ok(entry) = Entry::from_dir_entry(entry) {
                        // if descend into node_modules = false, return entry & stop descending
                        // TODO: Refactor naming
                        if !self.options.descend_nm && utils::is_node_modules(&entry.abs_path) {
                            // self.pop();
                            return Some(Ok(entry));
                        }

                        // TODO: Ignore hidden

                        // If dir && not a symlink, descend and return folder
                        if entry.file_type().is_dir() && !entry.file_type().is_symlink() {
                            if let Ok(read_dir) = fs::read_dir(&entry.abs_path) {
                                self.push(read_dir);

                                // TODO: option to include folder
                                return Some(Ok(entry));
                            }
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
    options: EntryOptions,
}

impl EntryWalk {
    pub fn new(path: PathBuf, opts: EntryOptions) -> Result<Self> {
        let list = fs::read_dir(path)?;
        Ok(EntryWalk {
            list: vec![list],
            options: opts,
        })
    }

    pub fn from_list(list: Vec<Entry>, opts: EntryOptions) -> Result<Self> {
        let collection = list
            .into_iter()
            .map(|e| fs::read_dir(e.path()))
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(EntryWalk {
            list: collection,
            options: opts,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions {
    ignore_hidden: bool,
    include_folders: bool,
    descend_nm: bool,
}

impl EntryOptions {
    pub fn new(ignore_hidden: bool, include_folders: bool, descend_nm: bool) -> Self {
        EntryOptions {
            ignore_hidden,
            include_folders,
            descend_nm,
        }
    }
}

impl IntoIterator for EntryWalk {
    type Item = Result<Entry>;
    type IntoIter = EntryIter;

    fn into_iter(self) -> Self::IntoIter {
        EntryIter {
            list: self.list,
            options: self.options,
        }
    }
}
