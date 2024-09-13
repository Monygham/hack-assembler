use std::io::{self};

mod command;
mod parser;
mod coder;
use parser::Parser;

fn main() -> io::Result<()> {
    let path_to_asm = "/home/lkomputer/Documents/Code/nand2tetris/projects/6/max/Max.asm";
    let mut parser = Parser::new(path_to_asm.to_string())?;
    parser.parse();
    Ok(())
}
