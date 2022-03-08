use std::{fs, io::Result, path::Path};

use prost_types::FileDescriptorProto;

/// Get the `.proto` file's relative path from file descriptor proto.
pub fn get_file_path(file: &FileDescriptorProto) -> Vec<String> {
    file.name()
        .strip_suffix(".proto")
        .unwrap()
        .split('/')
        .map(String::from)
        .collect()
}

/// Use this function to avoid uselessly touching the codegen file, causing
/// cache miss.
pub fn write_on_diff(path: impl AsRef<Path>, data: &[u8]) -> Result<()> {
    if data != fs::read(path.as_ref()).unwrap_or_default() {
        fs::write(path, data)?;
    }
    Ok(())
}
