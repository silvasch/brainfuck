use crate::{Error, Instruction};

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub jumps: Vec<(u32, u32)>,
}

impl Program {
    pub fn compile(program: &str) -> Result<Self, Error> {
        let mut instructions = vec![];
        let mut jumps = vec![];

        let mut open_jumps = vec![];

        for (i, ch) in program
            .chars()
            .filter(|value| ['<', '>', '+', '-', '.', '[', ']'].contains(value))
            .enumerate()
        {
            let instruction = ch.try_into()?;

            match instruction {
                Instruction::JumpForwards => open_jumps.push(i as u32),
                Instruction::JumpBackwards => {
                    let forwards_jump = open_jumps
                        .last()
                        .ok_or(Error::NoMatchingForwardsJump(i as u32))?;
                    jumps.push((*forwards_jump, i as u32));
                    open_jumps.pop();
                }
                _ => {}
            }

            instructions.push(instruction);
        }

        if !open_jumps.is_empty() {
            return Err(Error::NoMatchingBackwardsJump(
                *open_jumps
                    .last()
                    .expect("open_jumps is not empty in this path"),
            ));
        }

        Ok(Self {
            instructions,
            jumps,
        })
    }
}
