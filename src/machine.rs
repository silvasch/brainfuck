use std::collections::HashMap;

use crate::Instruction;

pub struct Machine {
    data: HashMap<u32, u32>,
    data_pointer: u32,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            data: HashMap::new(),
            data_pointer: 0,
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) -> Option<char> {
        match instruction {
            Instruction::MoveLeft => self.move_left(),
            Instruction::MoveRight => self.move_right(),
            Instruction::Increment => self.increment(),
            Instruction::Decrement => self.decrement(),
            Instruction::Output => return Some(self.output()),
        }

        None
    }

    fn move_left(&mut self) {
        if self.data_pointer == 0 {
            return;
        }

        self.data_pointer -= 1;
    }

    fn move_right(&mut self) {
        if self.data_pointer == u32::MAX {
            return;
        }

        self.data_pointer += 1;
    }

    fn increment(&mut self) {
        let initial = self.get_data();
        self.data.insert(self.data_pointer, initial + 1);
    }

    fn decrement(&mut self) {
        let initial = self.get_data();
        self.data.insert(self.data_pointer, initial - 1);
    }

    fn output(&mut self) -> char {
        char::from_u32(self.get_data()).unwrap()
    }

    fn get_data(&mut self) -> u32 {
        *self.data.get(&self.data_pointer).unwrap_or(&0)
    }
}
