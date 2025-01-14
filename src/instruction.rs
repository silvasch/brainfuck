use crate::Error;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Output,
    Input,
    JumpForwards,
    JumpBackwards,
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
            ',' => Ok(Instruction::Input),
            '[' => Ok(Instruction::JumpForwards),
            ']' => Ok(Instruction::JumpBackwards),
            ch => Err(Error::InvalidInstructionChar(ch)),
        }
    }
}
