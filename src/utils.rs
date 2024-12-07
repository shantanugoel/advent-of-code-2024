use std::io::BufRead;

pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = std::fs::File::open(file_path).unwrap();
    std::io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
}

#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    Integer32(i32),
    Integer64(i64),
    UInteger32(u32),
    UInteger64(u64),
}

impl From<i32> for Answer {
    fn from(i: i32) -> Self {
        Answer::Integer32(i)
    }
}

impl From<u32> for Answer {
    fn from(i: u32) -> Self {
        Answer::UInteger32(i)
    }
}

impl From<u64> for Answer {
    fn from(i: u64) -> Self {
        Answer::UInteger64(i)
    }
}

impl From<i64> for Answer {
    fn from(i: i64) -> Self {
        Answer::Integer64(i)
    }
}

impl From<usize> for Answer {
    fn from(i: usize) -> Self {
        Answer::Integer32(i as i32)
    }
}
