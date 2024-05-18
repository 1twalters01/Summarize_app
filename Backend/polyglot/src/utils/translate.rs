use std::{
    ffi::OsStr,
    path::Path,
};

pub fn get_name_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
    .file_name()
    .and_then(OsStr::to_str)
}

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
    .extension()
    .and_then(OsStr::to_str)
}

