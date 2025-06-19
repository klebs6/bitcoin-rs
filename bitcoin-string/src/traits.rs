crate::ix!();

pub trait ToUpper {
    fn to_upper(&self) -> Self;
}

pub trait ToLower {
    fn to_lower(&self) -> Self;
}
