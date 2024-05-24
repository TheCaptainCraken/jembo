use std::{fs, path::PathBuf};

pub fn get_music_files(path: PathBuf) -> Vec<PathBuf> {
    let folder = fs::read_dir(path).expect("Impossible to use this path.");
    let mut file_paths: Vec<PathBuf> = Vec::new();

    for file in folder {
        file_paths.push(file.expect("Impossible to access this file.").path());
    }

    file_paths
}

pub fn stringify_path(path: &PathBuf) -> String {
    path.to_str()
        .expect("Unable to convert path to string")
        .to_owned()
}

pub fn stringify_paths(paths: &Vec<PathBuf>) -> Vec<String> {
    paths.iter().map(|path| stringify_path(path)).collect()
}
