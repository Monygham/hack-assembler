pub mod commands;
use commands::ACommand;
use commands::CCommand;
use commands::LCommand;

pub enum Command {
    Comment,
    CCommand(CCommand),
    ACommand(ACommand),
    LCommand(LCommand),
}

impl Command {
    pub fn new(preprocessed_line: &String) -> Self {
        if Self::is_a_command(preprocessed_line) {
            Command::ACommand(ACommand::new(preprocessed_line))
        } else if Self::is_c_command(preprocessed_line) {
            Command::CCommand(CCommand::new(preprocessed_line))
        } else if Self::is_l_command(preprocessed_line) {
            Command::LCommand(LCommand::new(preprocessed_line))
        } else {
            Command::Comment
        }
    }

    fn is_a_command(preprocessed_line: &String) -> bool {
        preprocessed_line.starts_with("@")
    }

    fn is_c_command(preprocessed_line: &String) -> bool {
        ["A", "M", "D", "0"]
            .iter()
            .any(|v| preprocessed_line.starts_with(v))
    }

    fn is_l_command(preprocessed_line: &String) -> bool {
        preprocessed_line.starts_with("(")
    }
}
