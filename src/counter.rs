use std::fs;
use std::io;

pub struct CountResult {
    pub lines: usize,
    pub words: usize,
    pub characters: usize,
}

pub fn count_file(path: &str) -> Result<CountResult, io::Error> {
    let content = fs::read_to_string(path)?;

    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let characters = content.chars().count();

    Ok(CountResult {
        lines,
        words,
        characters,
    })
}
