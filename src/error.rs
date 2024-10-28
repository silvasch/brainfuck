#[derive(Debug)]
pub enum Error {
    InvalidInstructionChar(char),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInstructionChar(invalid_instruction_char) => write!(
                f,
                "'{}' is not a valid instruction character.",
                invalid_instruction_char
            ),
        }
    }
}

impl std::error::Error for Error {}
