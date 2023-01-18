crate::ix!();

#[repr(u8)]
#[derive(Clone,Hash,PartialEq,Eq)]
pub enum BlockFilterType 
{
    BASIC   = 0,
    INVALID = 255,
}

impl From<u8> for BlockFilterType {
    fn from(x: u8) -> Self {
        match x {
            0 => BlockFilterType::BASIC,
            _ => BlockFilterType::INVALID,
        }
    }
}

impl Default for BlockFilterType {
    fn default() -> Self { BlockFilterType::INVALID }
}
