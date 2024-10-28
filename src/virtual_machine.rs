use std::{collections::HashMap, io::Write};

use crate::{Error, Instruction, Program};

pub struct VirtualMachine {
    program: Program,

    data: HashMap<u32, u32>,
    data_pointer: u32,

    instruction_pointer: u32,

    terminal: console::Term,
}

impl VirtualMachine {
    pub fn new(program: Program) -> Self {
        Self {
            program,

            data: HashMap::new(),
            data_pointer: 0,

            instruction_pointer: 0,

            terminal: console::Term::stdout(),
        }
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        loop {
            if self.instruction_pointer as usize >= self.program.instructions.len() {
                break;
            }

            let instruction = self
                .program
                .instructions
                .get(self.instruction_pointer as usize)
                .expect("instruction pointer should never move out of range");

            match instruction {
                Instruction::MoveLeft => self.move_left(),
                Instruction::MoveRight => self.move_right(),
                Instruction::Increment => self.increment(),
                Instruction::Decrement => self.decrement(),
                Instruction::Output => self.output(),
                Instruction::Input => self.input(),
                Instruction::JumpForwards => self.jump_forwards(),
                Instruction::JumpBackwards => self.jump_backwards(),
            }

            self.instruction_pointer += 1;
        }

        Ok(())
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

    fn output(&mut self) {
        self.terminal.write_all(&[self.get_data() as u8]).unwrap();
    }

    fn input(&mut self) {
        let input = self.terminal.read_char().unwrap();
        self.terminal
            .write_all(input.to_string().as_bytes())
            .unwrap();
        let input = ascii_converter::string_to_decimals(&input.to_string()).unwrap()[0] as u32;
        self.data.insert(self.data_pointer, input);
    }

    fn jump_forwards(&mut self) {
        if self.get_data() != 0 {
            return;
        }

        for jump in &self.program.jumps {
            if jump.0 == self.instruction_pointer {
                self.instruction_pointer = jump.1;
                return;
            }
        }

        todo!()
    }

    fn jump_backwards(&mut self) {
        if self.get_data() == 0 {
            return;
        }

        for jump in &self.program.jumps {
            if jump.1 == self.instruction_pointer {
                self.instruction_pointer = jump.0;
                return;
            }
        }

        todo!()
    }

    fn get_data(&self) -> u32 {
        *self.data.get(&self.data_pointer).unwrap_or(&0)
    }
}
