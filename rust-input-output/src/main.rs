use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path: String =
        env::var("INPUT_FILE_PATH").unwrap_or("/mnt/input.txt".to_string());
    let output_file_path: String =
        env::var("OUTPUT_FILE_PATH").unwrap_or("/mnt/input.txt".to_string());

    let data = fs::read(input_file_path).unwrap();
    fs::write(output_file_path, data).unwrap()
}
