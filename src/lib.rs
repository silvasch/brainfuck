mod error;
pub use error::Error;

mod instruction;
pub(crate) use instruction::Instruction;

mod program;
pub(crate) use program::Program;

mod virtual_machine;
pub(crate) use virtual_machine::VirtualMachine;

pub fn run(program: &str) -> Result<(), Error> {
    let program = Program::compile(program)?;
    let mut virtual_machine = VirtualMachine::new(program);
    virtual_machine.execute()?;
    Ok(())
}
