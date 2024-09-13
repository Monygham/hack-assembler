use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::coder::{encode_address_command, encode_computation_command};
use crate::command::{ParsedLine, ParsedCommand, CCommand};

pub struct Parser {
    path_to_asm: String,
    symbol_table: HashMap<String, usize>,
    free_memory_address: usize,
}

impl Parser {
    pub fn new(path_to_asm: String) -> io::Result<Self> {
        let mut parser = Parser {
            path_to_asm,
            symbol_table: HashMap::from([
                ("R0".to_string(), 0),
                ("R1".to_string(), 1),
                ("R2".to_string(), 2),
                ("R3".to_string(), 3),
                ("R4".to_string(), 4),
                ("R5".to_string(), 5),
                ("R6".to_string(), 6),
                ("R7".to_string(), 7),
                ("R8".to_string(), 8),
                ("R9".to_string(), 9),
                ("R10".to_string(), 10),
                ("R11".to_string(), 11),
                ("R12".to_string(), 12),
                ("R13".to_string(), 13),
                ("R14".to_string(), 14),
                ("R15".to_string(), 15),
                ("SCREEN".to_string(), 16384),
                ("KBD".to_string(), 24576),
                ("SP".to_string(), 0),
                ("LCL".to_string(), 1),
                ("ARG".to_string(), 2),
                ("THIS".to_string(), 3),
                ("THAT".to_string(), 4),
            ]),
            free_memory_address: 16,
        };
        parser.parse_labels()?;
        Ok(parser)
    }

    pub fn parse(&mut self) {
        self.parse_commands();
    }
}

impl Parser {
    fn parse_commands(&mut self) -> io::Result<()> {
        let preprocessed_lines = self.preprocessed_lines_iterator()?;
        preprocessed_lines.for_each(|preprocessed_line| match ParsedLine::new(&preprocessed_line) {
            ParsedLine::Label(label) => {
                println!("{:}", preprocessed_line);
                println!(
                    "L-command {} in line number {}",
                    label,
                    self.get_symbol(&label).unwrap()
                );
            }
            ParsedLine::Address(address) => {
                println!("{:}", preprocessed_line);
                println!("Address is {:}", address);
            }
            ParsedLine::Symbol(symbol) => {
                        self.insert_symbol(symbol.clone());
                        println!(
                            "Symbol {} alocates variable at address {}",
                            symbol,
                            self.get_symbol(&symbol).unwrap()
                        );
                    
                let address = self.eliminate_symbol(symbol);
                println!("{:}", encode_address_command(address));
            }
            ParsedLine::Computation(command) => {
                println!("{:}", preprocessed_line);
                println!("dest is {}, comp is {}, jmp is {}", command.dest, command.comp, command.jmp);
                println!("{:}", encode_computation_command(command));
            }
            ParsedLine::Comment => {}
        });
        Ok(())
    }

    fn eliminate_symbol(&mut self, symbol: String) -> usize {
            self.insert_symbol(symbol.clone());
            self.get_symbol(&symbol).unwrap()
    }

    fn parse_labels(&mut self) -> io::Result<()> {
        let mut program_line_counter = 0;
        let preprocessed_lines = self.preprocessed_lines_iterator()?;
        preprocessed_lines.for_each(|preprocessed_line| match ParsedLine::new(&preprocessed_line) {
            ParsedLine::Label(label) => {
                self.insert_label(label, program_line_counter);
            }
            ParsedLine::Comment => {}
            _ => {
                program_line_counter += 1;
            }
        });
        Ok(())
    }

    fn insert_label(&mut self, label: String, line_number: usize) {
        self.symbol_table.insert(label, line_number);
    }
    fn insert_symbol(&mut self, symbol: String) {
        if let None = self.get_symbol(&symbol) {
            self.symbol_table.insert(symbol, self.free_memory_address);
            self.free_memory_address += 1;
        }
    }

    fn get_symbol(&self, symbol: &String) -> Option<usize> {
        self.symbol_table.get(symbol).copied()
    }

    fn preprocessed_lines_iterator(&self) -> io::Result<impl Iterator<Item = String>> {
        Ok(BufReader::new(File::open(&self.path_to_asm)?)
            .lines()
            .flatten()
            .map(|raw_line| Self::preprocess_raw_line(raw_line)))
    }
}

impl Parser {
    fn preprocess_raw_line(raw_line: String) -> String {
        let mut preprocessed_line = raw_line.trim().to_string();
        if let Some(i) = preprocessed_line.find(" ") {
            preprocessed_line = preprocessed_line[..i].to_string();
        };
        if let Some(i) = preprocessed_line.find("\\") {
            preprocessed_line = preprocessed_line[..i].to_string();
        };
        preprocessed_line
    }
}
