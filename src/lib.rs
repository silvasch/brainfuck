mod machine;
pub(crate) use machine::Machine;

mod error;
pub use error::Error;

mod instruction;
pub(crate) use instruction::Instruction;

pub fn run(program: &str) -> Result<(), Error> {
    let mut machine = Machine::new();

    for ch in program.chars() {
        let instruction = ch.try_into()?;
        let output = machine.execute_instruction(instruction);
        if let Some(output) = output {
            print!("{}", output);
        }
    }

    Ok(())
}
