use pinocchio::program_error::ProgramError;

pub mod make;
pub mod refund;
pub mod take;

pub use make::*;
pub use refund::*;
pub use take::*;

#[repr(u8)]
pub enum MyProgramInstruction {
    Make,
    Take,
    Refund,
}

impl TryFrom<&u8> for MyProgramInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MyProgramInstruction::Make),
            1 => Ok(MyProgramInstruction::Take),
            2 => Ok(MyProgramInstruction::Refund),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
