use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod coder;
mod command;
mod parser;
use coder::Coder;
use parser::Parser;

fn lines_in_sourcecode(path_to_asm: &str) -> Vec<String> {
    let err_msg = format!("Path {} does not exist", path_to_asm);
    let file = File::open(path_to_asm).expect(&err_msg);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() -> io::Result<()> {
    let path_to_asm = "/home/lkomputer/Documents/Code/nand2tetris/projects/6/max/Max.asm";
    let lines_in_sourcecode = lines_in_sourcecode(path_to_asm);
    let mut parser = Parser::new(lines_in_sourcecode);
    parser.parse_labels();
    let parsed_commands = parser.parse_commands();
    let coder = Coder::new(parsed_commands);
    let binary_code = coder.assemble_binary_code();

    Ok(())
}
