use std::env;
use std::path::{Path, PathBuf};
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

pub fn get_tmp_file_path(prefix: &str) -> PathBuf {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16) // Generate 16 random characters
        .map(char::from)
        .collect();

    let file_name = format!("{}.tmp_bash.sh", random_string);

    Path::new(prefix).join(file_name)
}