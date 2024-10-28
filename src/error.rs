#[derive(Debug)]
pub enum Error {
    InvalidInstructionChar(char),
    NoMatchingForwardsJump(u32),
    NoMatchingBackwardsJump(u32),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInstructionChar(ch) => write!(f, "'{}' is not a valid instruction.", ch),
            Error::NoMatchingForwardsJump(index) => write!(
                f,
                "the backwards jump at the index {} does not have a matching forwards jump.",
                index
            ),
            Error::NoMatchingBackwardsJump(index) => write!(
                f,
                "the forwards jump at the index {} does not have a matching backwards jump.",
                index
            ),
        }
    }
}

impl std::error::Error for Error {}
