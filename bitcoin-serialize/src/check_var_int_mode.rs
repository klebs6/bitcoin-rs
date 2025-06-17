crate::ix!();

pub struct CheckVarIntMode<const Mode: VarIntMode> {

}

use num_traits::{Signed, Unsigned};

trait ModeConstraint<const M: VarIntMode, I> {}
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

