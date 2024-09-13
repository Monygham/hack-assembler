pub enum ParsedCommand {
    Computation(CCommand),
    Address(usize),
}

pub enum ParsedLine {
    Comment,
    Computation(CCommand),
    Address(usize),
    Symbol(String),
    Label(String),
}

pub struct CCommand {
    pub dest: String,
    pub comp: String,
    pub jmp: String,
}


impl ParsedLine {
    pub fn new(preprocessed_line: &String) -> Self {
        if Self::is_address_or_symbol(preprocessed_line) {
            Self::parse_address_or_symbol(preprocessed_line)
        } else if Self::is_computation(preprocessed_line) {
            Self::parse_computation(preprocessed_line)
        } else if Self::is_label(preprocessed_line) {
            Self::parse_label(preprocessed_line)
        } else {
            Self::Comment
        }
    }

    fn parse_address_or_symbol(preprocessed_line: &String) -> Self {
        let address_string = preprocessed_line[1..].to_string();
        match address_string.parse::<usize>() {
            Ok(i) => Self::Address(i),
            Err(..) => Self::Symbol(address_string),
        }
    }

    fn parse_computation(preprocessed_line: &String) -> Self {
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
        Self::Computation(CCommand { dest, comp, jmp })
    }

    fn parse_label(preprocessed_line: &String) -> Self {
        let label_string = preprocessed_line
            .find(")")
            .map(|i| preprocessed_line[1..i].to_owned())
            .expect("Parse error for L-command");
        Self::Label(label_string)
    }

    fn is_address_or_symbol(preprocessed_line: &String) -> bool {
        preprocessed_line.starts_with("@")
    }

    fn is_computation(preprocessed_line: &String) -> bool {
        ["A", "M", "D", "0"]
            .iter()
            .any(|v| preprocessed_line.starts_with(v))
    }

    fn is_label(preprocessed_line: &String) -> bool {
        preprocessed_line.starts_with("(")
    }
}
