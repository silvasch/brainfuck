use crate::Error;

pub enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Output,
}

impl TryFrom<char> for Instruction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Instruction::MoveLeft),
            '>' => Ok(Instruction::MoveRight),
            '+' => Ok(Instruction::Increment),
            '-' => Ok(Instruction::Decrement),
            '.' => Ok(Instruction::Output),
            ch => Err(Error::InvalidInstructionChar(ch)),
        }
    }
}
