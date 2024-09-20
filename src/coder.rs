use crate::command::CCommand;
use crate::command::ParsedCommand;

pub struct Coder {
    parsed_commands: Vec<ParsedCommand>,
}

impl Coder {
    pub fn new(parsed_commands: Vec<ParsedCommand>) -> Self {
        Coder { parsed_commands }
    }

    pub fn assemble_binary_code(self) -> Vec<String> {
        self.parsed_commands
            .into_iter()
            .map(|parsed_command| match parsed_command {
                ParsedCommand::Address(address) => Self::encode_address_command(address),
                ParsedCommand::Computation(command) => Self::encode_computation_command(command),
            })
            .collect()
    }
}

impl Coder {
    fn encode_computation_command(command: CCommand) -> String {
        let comp_binary_code = match command.comp.as_str() {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            _ => panic!("Incorrect computation"),
        };
        let dest_binary_code = match command.dest.as_str() {
            "null" => "000",
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            _ => panic!("Incorrect destination"),
        };
        let jmp_binary_code = match command.jmp.as_str() {
            "null" => "000",
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => panic!("Incorrect jump"),
        };
        let mut command_binary_code = "111".to_string();
        command_binary_code.push_str(comp_binary_code);
        command_binary_code.push_str(dest_binary_code);
        command_binary_code.push_str(jmp_binary_code);
        command_binary_code
    }

    fn encode_address_command(address: usize) -> String {
        format!("{:0>16}", format!("{address:b}"))
    }
}

pub fn encode_computation_command(command: CCommand) -> String {
    let dest_binary_code = match command.dest.as_str() {
        "null" => "000",
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        _ => panic!("Incorrect destination"),
    };
    let comp_binary_code = match command.comp.as_str() {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => panic!("Incorrect computation"),
    };
    let jmp_binary_code = match command.jmp.as_str() {
        "null" => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => panic!("Incorrect jump"),
    };
    let mut command_binary_code = "111".to_string();
    command_binary_code.push_str(dest_binary_code);
    command_binary_code.push_str(comp_binary_code);
    command_binary_code.push_str(jmp_binary_code);
    command_binary_code
}

pub fn encode_address_command(address: usize) -> String {
    format!("{:0>16}", format!("{address:b}"))
}
