use std::{fs, path::PathBuf};

pub fn get_music_files(path: PathBuf) -> Vec<PathBuf> {
    let folder = fs::read_dir(path).expect("Impossible to use this path.");
    let mut file_paths: Vec<PathBuf> = Vec::new();

    for file in folder {
        file_paths.push(file.expect("Impossible to access this file.").path());
    }

    file_paths
}
