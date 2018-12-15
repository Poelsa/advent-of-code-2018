use std::fs::File;
use std::io::Read;

pub fn get_string_input(filename: &str) -> Vec<String> {
    let input_string = read_file_to_string(filename);
    let strings: Vec<_> = input_string.split("\n").collect();
    let mut result: Vec<String> = Vec::new();
    for line in strings {
        result.push(line.to_string());
    }
    return result;
}

fn read_file_to_string(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut result = String::new();
    f.read_to_string(&mut result).expect("could not read file");
    return result;
}