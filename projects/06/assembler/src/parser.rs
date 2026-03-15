use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::path::Path;

pub struct Parser {
    command_iter: Peekable<Lines<BufReader<File>>>,
}

impl Parser {
    pub fn new(filename: &Path) -> Self {
        let f = File::open(filename).unwrap();
        let buf_reader = BufReader::new(f);
        let command_iter = buf_reader.lines().peekable();
        Self { command_iter }
    }

    pub fn has_more_commands(&mut self) -> bool {
        match self.command_iter.peek() {
            None => false,
            Some(_) => true
        }            
    }
}

