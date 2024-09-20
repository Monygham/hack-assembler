use std::fs::File;
use std::io::{self, Write, BufRead, BufReader, BufWriter};
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

fn write_binary_code_to_file(binary_code: Vec<String>, path_to_binary: String) -> io::Result<()> {
    let file = File::create(path_to_binary).expect("");
    let mut writer = BufWriter::new(file);
    for code_line in binary_code.iter() {
        writeln!(writer, "{}", code_line)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let path_to_asm = "/home/lkomputer/Documents/Code/nand2tetris/projects/6/pong/Pong.asm";
    let lines_in_sourcecode = lines_in_sourcecode(path_to_asm);
    let mut parser = Parser::new(lines_in_sourcecode);
    parser.parse_labels();
    // parser.parse();
    let parsed_commands = parser.parse_commands();
    let coder = Coder::new(parsed_commands);
    let binary_code = coder.assemble_binary_code();
    write_binary_code_to_file(binary_code, "result.hack".to_string())?;
    Ok(())
}
