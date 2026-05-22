#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    String(String),
    Variable(String),
    Concat { parts: Vec<Arg>, lexeme: String },
}

impl Arg {
    pub fn raw_value(&self) -> &str {
        match self {
            Arg::String(s) => s,
            Arg::Variable(name) => name,
            Arg::Concat { lexeme, .. } => lexeme,
        }
    }

    fn concat_lexeme(parts: &[Arg]) -> String {
        parts
            .iter()
            .map(|part| match part {
                Arg::String(s) => s.clone(),
                Arg::Variable(name) => format!("${name}"),
                Arg::Concat { lexeme, .. } => lexeme.clone(),
            })
            .collect()
    }

    pub fn concat(parts: Vec<Arg>) -> Self {
        let lexeme = Self::concat_lexeme(&parts);
        Arg::Concat { parts, lexeme }
    }
}