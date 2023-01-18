crate::ix!();

pub const INVALID: u32 = 0xFFFFFFFF;

#[repr(u32)]
pub enum Instruction {
    RETURN  = 0,
    JUMP    = 1,
    MATCH   = 2,
    DEFAULT = 3,
}

impl Instruction {

    pub fn new(x: u32) -> Self {
        match x {
            0 => Instruction::RETURN,
            1 => Instruction::JUMP,
            2 => Instruction::MATCH,
            3 => Instruction::DEFAULT,
            _ => unreachable!(),
        }
    }
}

