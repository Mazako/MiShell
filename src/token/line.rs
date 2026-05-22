use super::input::Input;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub input: Input,
    pub pipes: Vec<Input>,
    pub background: bool,
}
