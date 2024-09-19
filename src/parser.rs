use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::coder::{encode_address_command, encode_computation_command};
use crate::command::{CCommand, ParsedCommand, ParsedLine};

struct SymbolTable {
    table: HashMap<String, usize>,
    free_memory_address: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: HashMap::from([
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
        }
    }

    pub fn insert_symbol(&mut self, symbol: String) {
        if let None = self.get_symbol(&symbol) {
            self.table.insert(symbol, self.free_memory_address);
            self.free_memory_address += 1;
        }
    }

    pub fn get_symbol(&self, symbol: &String) -> Option<usize> {
        self.table.get(symbol).copied()
    }

    pub fn insert_label(&mut self, label: String, line_number: usize) {
        self.table.insert(label, line_number);
    }

    pub fn eliminate_symbol(&mut self, symbol: String) -> usize {
        self.insert_symbol(symbol.clone());
        self.get_symbol(&symbol).unwrap()
    }
}

pub struct Parser {
    preprocessed_lines: Vec<String>,
    symbol_table: SymbolTable,
}

impl Parser {
    pub fn new(lines_in_sourcecode: Vec<String>) -> Self {
        let preprocessed_lines: Vec<String> = lines_in_sourcecode
            .iter()
            .map(|raw_line| Self::preprocess_raw_line(raw_line))
            .collect();
        Parser {
            preprocessed_lines,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn parse_commands(&mut self) -> Vec<ParsedCommand> {
        let mut parsed_commands: Vec<ParsedCommand> = vec![];
        self.preprocessed_lines
            .iter()
            .for_each(
                |preprocessed_line| match ParsedLine::new(&preprocessed_line) {
                    ParsedLine::Address(address) => {
                        parsed_commands.push(ParsedCommand::Address(address));
                    }
                    ParsedLine::Symbol(symbol) => {
                        self.symbol_table.insert_symbol(symbol.clone());
                        let address = self.symbol_table.eliminate_symbol(symbol);
                        parsed_commands.push(ParsedCommand::Address(address));
                    }
                    ParsedLine::Computation(command) => {
                        parsed_commands.push(ParsedCommand::Computation(command));
                    }
                    _ => {}
                },
            );
        parsed_commands
    }

    pub fn parse_labels(&mut self) {
        let mut program_line_counter = 0;
        let preprocessed_lines_iter = self.preprocessed_lines.iter();
        preprocessed_lines_iter.for_each(|preprocessed_line| {
            match ParsedLine::new(&preprocessed_line) {
                ParsedLine::Label(label) => {
                    self.symbol_table.insert_label(label, program_line_counter);
                }
                ParsedLine::Comment => {}
                _ => {
                    program_line_counter += 1;
                }
            }
        });
    }
    
    pub fn parse(&mut self) {
        self.preprocessed_lines
            .iter()
            .for_each(
                |preprocessed_line| match ParsedLine::new(&preprocessed_line) {
                    ParsedLine::Label(label) => {
                        println!("{:}", preprocessed_line);
                        println!(
                            "L-command {} in line number {}",
                            label,
                            self.symbol_table.get_symbol(&label).unwrap()
                        );
                    }
                    ParsedLine::Address(address) => {
                        println!("{:}", preprocessed_line);
                        println!("Address is {:}", address);
                    }
                    ParsedLine::Symbol(symbol) => {
                        self.symbol_table.insert_symbol(symbol.clone());
                        println!(
                            "Symbol {} alocates variable at address {}",
                            symbol,
                            self.symbol_table.get_symbol(&symbol).unwrap()
                        );

                        let address = self.symbol_table.eliminate_symbol(symbol);
                        println!("{:}", encode_address_command(address));
                    }
                    ParsedLine::Computation(command) => {
                        println!("{:}", preprocessed_line);
                        println!(
                            "dest is {}, comp is {}, jmp is {}",
                            command.dest, command.comp, command.jmp
                        );
                        println!("{:}", encode_computation_command(command));
                    }
                    ParsedLine::Comment => {}
                },
            );
    }
}

impl Parser {
    fn preprocess_raw_line(raw_line: &String) -> String {
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
