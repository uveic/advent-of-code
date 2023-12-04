use std::fs;

// ToDo: figure it out
pub fn read_lines(filename :&str) -> Vec<&str> {
    let content = fs::read_to_string(String::from(filename)).unwrap();
    content.split("\n").filter(|l| l.len() > 0).collect()
}