use std::fs;
use crate::save;

pub fn open_save(path: &str) -> save::SaveFile {
    let data = fs::read(path).expect("Failed to read save file");
    save::SaveFile::load(data)
}

pub fn write_save(path: &str, save_file: &save::SaveFile) -> std::io::Result<()> {
    let data = save_file.save();
    fs::write(path, data)
}

pub fn does_file_exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}