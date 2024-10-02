use std::collections::HashMap;
use crate::command::{ParsedCommand, ParsedLine};

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

    pub fn get_symbol(&self, symbol: &String) -> Option<usize> {
        self.table.get(symbol).copied()
    }

    pub fn insert_label(&mut self, label: String, line_number: usize) {
        self.table.insert(label, line_number);
    }

    pub fn eliminate_symbol(&mut self, symbol: String) -> usize {
        if self.get_symbol(&symbol).is_none() {
            self.table.insert(symbol.clone(), self.free_memory_address);
            self.free_memory_address += 1;
        }
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
                |preprocessed_line| match ParsedLine::new(preprocessed_line) {
                    ParsedLine::Address(address) => {
                        parsed_commands.push(ParsedCommand::Address(address));
                    }
                    ParsedLine::Symbol(symbol) => {
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
            match ParsedLine::new(preprocessed_line) {
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
}

impl Parser {
    fn preprocess_raw_line(raw_line: &str) -> String {
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
