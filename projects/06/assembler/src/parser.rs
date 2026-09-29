use anyhow::Result;
use regex::Regex;
// use core::fmt;
// use std::{clone, print};
// use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::path::Path;
use std::fmt;


#[derive(Debug)]
struct ParserError;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command not recognised")
    }
}

impl std::error::Error for ParserError {}


#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
pub enum CommandTypeEnum {
    ACommand,
    CCommand,
    LCommand,
    EmptyCommand, // used for whole line comments and empty lines
}

#[derive(Debug)]
struct CommandType {
    command_type_enum: CommandTypeEnum,
    command_matcher: Regex,
}

impl CommandType {
    fn new(command_type_enum: CommandTypeEnum, command_matcher: Regex) -> Self {
        Self {
            command_type_enum,
            command_matcher,
        }
    }

    fn is_match(&mut self, command: &str) -> Option<CommandTypeEnum> {
        if self.command_matcher.is_match(command) {
            return Some(self.command_type_enum);
        }
        None
    }
}

pub struct Parser {
    command_iter: Peekable<Lines<BufReader<File>>>,
    command_current: Option<Result<std::string::String, std::io::Error>>,
    re_command_cleaner: Regex,
    command_types: [CommandType; 3],
}

impl Parser {
    pub fn new(filename: &Path) -> Self {
        let f = File::open(filename).unwrap();
        let buf_reader = BufReader::new(f);
        let command_iter = buf_reader.lines().peekable();
        let command_current = None;
        let re_command_cleaner = Regex::new(r"^\s*(.*?)(?:\s*//.*)?$").unwrap(); // https://regex101.com/?regex=%5E%5Cs*%28.*%3F%29%28%3F%3A%5Cs*%2F%2F.*%29%3F%24&testString=+strip+space+at+start%0Astrip+space+at+end++++%0Akeep+space+in+middle%0Acommandlinewithnocomments%0Acommand%2F%2Fcomments%0Aa+%2B+more+%2F+%3D+complex+%21+command%0Acommand+%2F%2F+comment%0Acommand%2F%2Fcomments%2F%2Fstillcomments%2F%2Fevenmorecomments%0A%40command%2Fmorecommand%2F%2Fcomments%0A%2F%2Fcomments%0A&flags=gmu&flavor=rust&delimiter=%22
        let command_types = [
            CommandType::new(
                CommandTypeEnum::EmptyCommand,
                Regex::new(r"^$").unwrap()
            ),
            CommandType::new(
                CommandTypeEnum::ACommand,
                Regex::new(r"^@[\d]*$|^@[a-zA-Z_]*$").unwrap() // https://regex101.com/?regex=%5E%40%5B%5Cd%5D*%24%7C%5E%40%5Ba-zA-Z_%5D*%24&testString=%40123%0A%40symbol%0A%40SYMBOL%0A%40sYbMoL%0A%40symbol_with_underscores%0A%40numbers10andletters%0A_%40thisshouldntmatch&flags=gmu&flavor=rust&delimiter=%22 a_matcher
            ),
            CommandType::new(
                CommandTypeEnum::CCommand,
                // not an exhaustive c command matcher - validation will come when fully parsing the command
                Regex::new(r"^(?:AMD|MD|AD|AM|A|D|M)=(?:.+)$").unwrap() // https://regex101.com/?regex=%5E%28%3F%3AAMD%7CMD%7CAD%7CAM%7CA%7CD%7CM%29%3D%28%3F%3A.%2B%29%24&testString=M%3DM%2B1%0AD%3DM%0AD%3DD-A%0AM%3D0%0AM%3D999%0AM%3DM%2B1%0AD%3DM%2B1%0AMD%3DM%2B1%0AA%3DM%2B1%0AAM%3DM%2B1%0AAD%3DM%2B1%0AAMD%3DM%2B1%0AA%3DM-D%0A%40shouldnotmatch%0Aamd%3D%0AO%3D%0AAA%3D%0A0%3B%0A%0A%0A%0A&flags=gmu&flavor=rust&delimiter=%22
            ),
        ];
        // let l_command_detector =;
        Self {
            command_iter,
            command_current,
            re_command_cleaner,
            // a_matcher,
            // c_matcher,
            command_types,
            // l_command_detector,
        }
    }

    pub fn has_more_commands(&mut self) -> bool {
        match self.command_iter.peek() {
            None => false,
            Some(_) => true,
        }
    }

    // cleans the next command and loads it into command_current
    pub fn advance(&mut self) {
        self.command_current = self.command_iter.next().map(|result| {
            result.map(|command: String| {
                self.re_command_cleaner.captures(command.as_ref()).unwrap()[1].to_owned()
            })
        });
    }

    pub fn command_type(&mut self) -> Result<CommandTypeEnum, Box<dyn std::error::Error + '_>> {
        let has_more_commands = self.has_more_commands();
        match &self.command_current {
            Some(result) => match result {
                Ok(command) => {
                    println!("{command}");
                    let mut matches = self.command_types
                        .iter_mut()
                        .filter_map(|command_type| command_type.is_match(command));

                    // ensures one match only
                    let command_type: Option<CommandTypeEnum> = match (matches.next(), matches.next()) {
                        (Some(cmd), None) => Some(cmd),
                        _ => None,
                    };

                    command_type.ok_or_else(|| Box::new(ParserError) as Box<dyn std::error::Error>)
                }
                Err(e) => Err(Box::new(e)),
            }

            None => match has_more_commands {
                false => Err(format!("End of file reached").into()),
                true => Err(format!("Command not read").into()),
            },
        }
    }
}
