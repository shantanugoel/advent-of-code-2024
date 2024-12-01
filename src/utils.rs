use std::io::BufRead;

pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = std::fs::File::open(file_path).unwrap();
    std::io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
}