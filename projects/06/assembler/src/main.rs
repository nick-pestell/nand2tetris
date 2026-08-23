use std::path::Path;

use clap::Parser;

mod code;
mod parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    asm_filename: String,
}

fn main() {
    let args = Args::parse();

    let mut my_parser = parser::Parser::new(Path::new(&args.asm_filename));
    
    while my_parser.has_more_commands() {
        my_parser.advance();
        println!("{:?}", my_parser.command_type().unwrap());
        println!();
    }
}
