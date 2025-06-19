// ---------------- [ File: bitcoin-serialize/src/check_var_int_mode.rs ]
crate::ix!();

pub struct CheckVarIntMode<const Mode: VarIntMode> {

}

use num_traits::{Signed, Unsigned};

pub trait ModeConstraint<const M: VarIntMode, I> {}
impl<I: Unsigned> ModeConstraint<{ VarIntMode::Default },           I> for () {}
impl<I: Signed>   ModeConstraint<{ VarIntMode::NonNegativeSigned }, I> for () {}

impl<const Mode: VarIntMode> CheckVarIntMode<Mode> {
    #[inline]
    pub fn new<I>() -> Self
    where
        (): ModeConstraint<Mode, I>,
    {
        trace!("CheckVarIntMode<Mode={:?}, I={}>", Mode, std::any::type_name::<I>());
        Self {}
    }
}

#[cfg(test)]
mod check_var_int_mode_tests {
    use super::*;

    /// `VarIntMode::Default` must accept **unsigned** integral types.
    #[traced_test]
    fn default_mode_accepts_unsigned() {
        // Compiles & runs ⇒ constraint holds.
        let _marker = CheckVarIntMode::<{ VarIntMode::Default }>::new::<u32>();
    }

    /// `VarIntMode::NonNegativeSigned` must accept **signed** integral
    /// types (historical Bitcoin Core behaviour).
    #[traced_test]
    fn legacy_mode_accepts_signed() {
        let _marker = CheckVarIntMode::<{ VarIntMode::NonNegativeSigned }>::new::<i64>();
    }
}
