use std::default;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::path::Path;

pub enum CommandType {
    A_COMMAND,
    C_COMMAND,
    L_COMMAND,    
}

pub struct Parser {
    command_iter: Peekable<Lines<BufReader<File>>>,
    command_current: Option<Result<std::string::String, std::io::Error>>, 
}

impl Parser {
    pub fn new(filename: &Path) -> Self {
        let f = File::open(filename).unwrap();
        let buf_reader = BufReader::new(f);
        let command_iter = buf_reader.lines().peekable();
        let command_current = None;
        Self { command_iter, command_current}
    }

    // this function isn't actually really needed since
    // advance(..) can handle the end of the iterator and
    // responds by returning None. 
    // I have included it because the API spec. in project 6
    // asks for it. 
    pub fn has_more_commands(&mut self) -> bool {
        match self.command_iter.peek() {
            None => false,
            Some(_) => true
        }            
    }

    pub fn advance(&mut self) {
        self.command_current = self.command_iter.next();
    }

    pub fn command_type(&mut self) -> Result<CommandType> {
        let a_matcher = '@';
        let c_matcher = "dest=";
        match self.command_current {
            Some(result) => {
                match result {
                    Ok(result) => {
                        let foo_iter = result.chars();
                        match foo_iter.next() {
                            None => Err("some error"),
                            Some(first_char) => {
                                match first_char {
                                    '@' => Ok(CommandType::A_COMMAND),
                                    'd' => {

                                    },
                                    default => 
                                }
                            }
                        } foo_iter.next() 
                        if result.chars().take(5).collect::<String>() == c_matcher {
                            return CommandType::C_COMMAND;
                        } else {
                            return CommandType::A_COMMAND;
                        }
                    }
                }
            }
        }
    }
}

