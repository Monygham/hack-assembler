use std::fmt;

pub struct CCommand {
    pub dest: String,
    pub comp: String,
    pub jmp: String,
}

impl CCommand {
    pub fn new(preprocessed_line: &String) -> Self {
        let mut assignement_idx: Option<usize> = None;
        if let Some(i) = preprocessed_line.find("=") {
            assignement_idx = Some(i);
        }

        let mut semicolon_idx: Option<usize> = None;
        if let Some(i) = preprocessed_line.find(";") {
            semicolon_idx = Some(i);
        }

        let (dest, comp, jmp) = match (assignement_idx, semicolon_idx) {
            (Some(i), Some(j)) => (
                preprocessed_line[0..i].to_owned(),
                preprocessed_line[i + 1..j].to_owned(),
                preprocessed_line[j + 1..].to_owned(),
            ),
            (Some(i), None) => (
                preprocessed_line[0..i].to_owned(),
                preprocessed_line[i + 1..].to_owned(),
                "null".to_owned(),
            ),
            (None, Some(j)) => (
                "null".to_string(),
                preprocessed_line[0..j].to_owned(),
                preprocessed_line[j + 1..].to_owned(),
            ),
            (_, _) => {
                panic!("C-command should either contain semicolon or assignement")
            }
        };
        CCommand { dest, comp, jmp }
    }
}

impl fmt::Display for CCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "destination: {}, computation: {}, jump: {}",
            self.dest, self.comp, self.jmp
        )
    }
}

pub enum ACommand {
    Address(usize),
    Symbol(String),
}
impl ACommand {
    pub fn new(preprocessed_line: &String) -> Self {
        let address = preprocessed_line[1..].to_string();
        match address.parse::<usize>() {
            Ok(i) => ACommand::Address(i),
            Err(..) => ACommand::Symbol(address),
        }
    }
}

pub struct LCommand {
    pub label: String,
}

impl LCommand {
    pub fn new(line: &String) -> Self {
        let label = line
            .find(")")
            .map(|i| line[1..i].to_owned())
            .expect("Parse error for L-command");
        LCommand { label }
    }
}
