use clap::Parser as ArgParser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

mod coder;
mod command;
mod parser;
use coder::Coder;
use parser::Parser;

#[derive(ArgParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path_to_asm: String,
    #[arg(short, long)]
    output_dirpath: Option<String>,
}

fn parse_command_line_args() -> (String, String) {
    let args = Args::parse();
    let path_to_asm = args.path_to_asm;
    if !Path::new(&path_to_asm).is_file() {
        panic!("Sourcefile not found.");
    }
    if !(Path::new(&path_to_asm).extension().unwrap().to_str() == Some("asm")) {
        panic!(
            "Sourcefile found but has incorrect extension. Required {}",
            ".asm"
        );
    }
    let mut filename_of_output = Path::new(&path_to_asm)
        .file_name()
        .expect("Sourcefile has no filename.")
        .to_str()
        .expect("Sourcefile should be string.")
        .to_string()
        .strip_suffix(".asm")
        .expect("Sourcefile should be .asm file")
        .to_string();
    filename_of_output.push_str(".hack");
    let mut output_dirpath = Path::new("./")
        .join(&filename_of_output)
        .to_str()
        .unwrap()
        .to_string();

    if let Some(output_dirpath_string) = args.output_dirpath {
        if Path::new(&output_dirpath_string).is_dir() {
            output_dirpath = Path::new(&output_dirpath_string)
                .join(&filename_of_output)
                .to_str()
                .unwrap()
                .to_string();
        }
    }
    (path_to_asm, output_dirpath)
}
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
    let (path_to_asm, path_to_hack) = parse_command_line_args();
    // let path_to_asm = "/home/lkomputer/Documents/Code/nand2tetris/projects/6/pong/Pong.asm";
    let lines_in_sourcecode = lines_in_sourcecode(&path_to_asm);
    let mut parser = Parser::new(lines_in_sourcecode);
    parser.parse_labels();
    // parser.parse();
    let parsed_commands = parser.parse_commands();
    let coder = Coder::new(parsed_commands);
    let binary_code = coder.assemble_binary_code();
    write_binary_code_to_file(binary_code, path_to_hack)?;
    Ok(())
}
