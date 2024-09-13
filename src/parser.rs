use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::IntoIterator;

use crate::coder::{encode_address_command, encode_computation_command};
use crate::command::commands::ACommand;
use crate::command::Command;

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
        preprocessed_lines.for_each(|preprocessed_line| match Command::new(&preprocessed_line) {
            Command::LCommand(command) => {
                println!("{:}", preprocessed_line);
                let label = command.label;
                println!(
                    "L-command {} in line number {}",
                    label,
                    self.get_symbol(&label).unwrap()
                );
            }
            Command::ACommand(command) => {
                println!("{:}", preprocessed_line);
                match command {
                    ACommand::Address(i) => {
                        println!("Address is {}", i);
                    }
                    ACommand::Symbol(ref s) => {
                        self.insert_symbol(s.clone());
                        println!(
                            "Symbol {} alocates variable at address {}",
                            s,
                            self.get_symbol(&s).unwrap()
                        );
                    }
                };
                let transformed_command = self.substitute_address(command);
                println!("{:}", encode_address_command(transformed_command));
            }
            Command::CCommand(command) => {
                println!("{:}", preprocessed_line);
                println!("{:}", command);
                println!("{:}", encode_computation_command(command));
            }
            Command::Comment => {}
        });
        Ok(())
    }

    fn substitute_address(&mut self, command: ACommand) -> ACommand {
        match command {
            ACommand::Address(_) => command,
            ACommand::Symbol(s) => {
                self.insert_symbol(s.clone());
                let address = self.get_symbol(&s).unwrap();
                ACommand::Address(address)
            }
        }
    }

    fn parse_labels(&mut self) -> io::Result<()> {
        let mut program_line_counter = 0;
        let preprocessed_lines = self.preprocessed_lines_iterator()?;
        preprocessed_lines.for_each(|preprocessed_line| match Command::new(&preprocessed_line) {
            Command::LCommand(command) => {
                let label = command.label;
                self.insert_label(label, program_line_counter);
            }
            Command::Comment => {}
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
