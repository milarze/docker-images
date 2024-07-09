use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let output_file_path: String = env::var("OUTPUT_FILE_PATH").unwrap();

    fs::write(output_file_path, args.first().unwrap()).unwrap()
}
