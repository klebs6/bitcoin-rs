// ---------------- [ File: bitcoin-u256/src/delegates.rs ]
crate::ix!();

impl Not for ArithU256 {
    type Output = ArithU256;

    fn not(self) -> ArithU256 {
        // bitwise NOT => ~self
        let mut ret = self.clone();
        // We rely on `BaseUInt256`'s `Not` (which returns a new `BaseUInt`).
        // But we want to do it in-place, so:
        ret.base = !ret.base;
        ret
    }
}

//------------------------------------//
// Bitwise XOR
//------------------------------------//
impl BitXor for ArithU256 {
    type Output = ArithU256;

    fn bitxor(mut self, rhs: ArithU256) -> ArithU256 {
        // We have `impl BitXorAssign<&BaseUInt256> for BaseUInt256`,
        // so let's call `self.base ^= &rhs.base`:
        self.base ^= &rhs.base;
        self
    }
}

impl BitXorAssign for ArithU256 {
    fn bitxor_assign(&mut self, rhs: ArithU256) {
        self.base ^= &rhs.base;
    }
}

impl BitXorAssign<u64> for ArithU256 {
    fn bitxor_assign(&mut self, rhs: u64) {
        // Self::base is a BaseUInt256, which holds 8 limbs of 32 bits each
        // We want to XOR the lower 32 bits with self.base.pn[0] and the upper 32 bits with self.base.pn[1].
        // Everything else remains unchanged.
        let lower_32 = (rhs & 0xFFFF_FFFF) as u32;
        let upper_32 = ((rhs >> 32) & 0xFFFF_FFFF) as u32;

        // XOR the low limb
        self.base.set_limb(0, self.get_limb(0) ^ lower_32);

        // XOR the next limb if we have at least two limbs
        if self.base.limb_count() > 1 {
            self.base.set_limb(1, self.base.get_limb(1) ^ upper_32);
        }
    }
}

impl BitXorAssign<&ArithU256> for ArithU256 {
    fn bitxor_assign(&mut self, rhs: &ArithU256) {
        self.base ^= &rhs.base;
    }
}

//------------------------------------//
// Bitwise AND
//------------------------------------//
impl BitAnd for ArithU256 {
    type Output = ArithU256;

    fn bitand(mut self, rhs: ArithU256) -> ArithU256 {
        self.base &= &rhs.base;
        self
    }
}

impl BitAndAssign for ArithU256 {
    fn bitand_assign(&mut self, rhs: ArithU256) {
        self.base &= &rhs.base;
    }
}

//------------------------------------//
// Bitwise OR
//------------------------------------//
impl BitOr for ArithU256 {
    type Output = ArithU256;

    fn bitor(mut self, rhs: ArithU256) -> ArithU256 {
        self.base |= &rhs.base;
        self
    }
}

impl BitOrAssign for ArithU256 {
    fn bitor_assign(&mut self, rhs: ArithU256) {
        self.base |= &rhs.base;
    }
}

//------------------------------------//
// Shl / ShlAssign
//------------------------------------//
impl Shl<u32> for ArithU256 {
    type Output = ArithU256;

    fn shl(mut self, shift: u32) -> ArithU256 {
        // We already have `impl ShlAssign<u32> for ArithU256`,
        // so just reuse it:
        self <<= shift;
        self
    }
}

// (already have ShlAssign<u32> for ArithU256 => self.base <<= shift)

//------------------------------------//
// Shr / ShrAssign
//------------------------------------//
impl Shr<u32> for ArithU256 {
    type Output = ArithU256;

    fn shr(mut self, shift: u32) -> ArithU256 {
        self >>= shift;
        self
    }
}

/////////////////////////////////////////
// 1) Neg (unary minus):  mod 2^256 2's complement
/////////////////////////////////////////
impl Neg for ArithU256 {
    type Output = ArithU256;

    fn neg(self) -> ArithU256 {
        // -x mod 2^256 is two's complement => ~x + 1
        let mut ret = !self; // uses Bitwise NOT on ArithU256 (we'll define that below)
        ret += 1u64;         // we need ArithU256 += u64
        ret
    }
}

/////////////////////////////////////////
// 2) Let’s allow "Add<&ArithU256>" => ArithU256
/////////////////////////////////////////
impl Add<&ArithU256> for ArithU256 {
    type Output = ArithU256;

    fn add(mut self, rhs: &ArithU256) -> ArithU256 {
        self.base += &rhs.base; // uses BaseUInt256::AddAssign<&BaseUInt256>
        self
    }
}

/////////////////////////////////////////
// 3) AddAssign<&ArithU256>, AddAssign<u64>
/////////////////////////////////////////
impl AddAssign<&ArithU256> for ArithU256 {
    fn add_assign(&mut self, rhs: &ArithU256) {
        self.base += &rhs.base;
    }
}
impl AddAssign<u64> for ArithU256 {
    fn add_assign(&mut self, rhs: u64) {
        self.base += rhs; // BaseUInt256 has AddAssign<u64>
    }
}

/////////////////////////////////////////
// 5) BitOrAssign for ArithU256
/////////////////////////////////////////
impl BitOrAssign<&ArithU256> for ArithU256 {
    fn bitor_assign(&mut self, rhs: &ArithU256) {
        self.base |= &rhs.base; 
    }
}
impl BitOrAssign<u64> for ArithU256 {
    fn bitor_assign(&mut self, rhs: u64) {
        self.base |= rhs;
    }
}


/////////////////////////////////////////
// 7) BitAndAssign for ArithU256
/////////////////////////////////////////
impl BitAndAssign<&ArithU256> for ArithU256 {
    fn bitand_assign(&mut self, rhs: &ArithU256) {
        self.base &= &rhs.base;
    }
}
impl BitAndAssign<u64> for ArithU256 {
    fn bitand_assign(&mut self, rhs: u64) {
        self.base &= rhs;
    }
}

// somewhere in your ArithU256 code:
use core::ops::{Mul, Div, Sub, SubAssign};

// 1) Multiplying ArithU256 by ArithU256
impl Mul<&ArithU256> for ArithU256 {
    type Output = ArithU256;
    fn mul(mut self, rhs: &ArithU256) -> ArithU256 {
        self *= rhs; // uses MulAssign<&ArithU256>
        self
    }
}
// optional convenience so you can do `r1 * r2` without referencing the right side:
impl Mul<ArithU256> for ArithU256 {
    type Output = ArithU256;
    fn mul(self, rhs: ArithU256) -> ArithU256 {
        // forward to mul(self, &rhs)
        self * &rhs
    }
}

// 2) Sub
impl Sub<&ArithU256> for ArithU256 {
    type Output = ArithU256;
    fn sub(mut self, rhs: &ArithU256) -> ArithU256 {
        // you DO have `self += &(-rhs.clone())` since we have Neg. 
        self += &(-rhs.clone());
        self
    }
}
impl Sub<ArithU256> for ArithU256 {
    type Output = ArithU256;
    fn sub(self, rhs: ArithU256) -> ArithU256 {
        self - &rhs
    }
}

// 3) Div
impl Div<&ArithU256> for ArithU256 {
    type Output = ArithU256;
    fn div(mut self, rhs: &ArithU256) -> ArithU256 {
        self /= rhs; // uses DivAssign<&ArithU256>
        self
    }
}
impl Div<ArithU256> for ArithU256 {
    type Output = ArithU256;
    fn div(self, rhs: ArithU256) -> ArithU256 {
        self / &rhs
    }
}

// 4) SubAssign<u64>
impl SubAssign<u64> for ArithU256 {
    fn sub_assign(&mut self, other: u64) {
        // you have AddAssign<u64>, but not SubAssign<u64>. 
        // so we can do:
        let tmp = ArithU256::from(other);
        *self += &(-tmp);
    }
}

// ---------------------------------------------------------
// 1) Multiply by u64
//    So you can do:  let prod = r1.clone() * 5u64;
//    or:             let prod = &r1 * 5u64;
// ---------------------------------------------------------
impl Mul<u64> for ArithU256 {
    type Output = ArithU256;

    fn mul(mut self, rhs: u64) -> Self::Output {
        // We rely on our existing `impl MulAssign<u32> for ArithU256` or similar.
        // But the incoming `rhs` is 64 bits. We'll do a small approach:
        // "self *= rhs" is only valid if you have `impl MulAssign<u64>` or do the splitting.
        // If you already have `impl MulAssign<u64>`, just do self *= rhs.
        // If not, we can do:
        self.base *= rhs; 
        self
    }
}

impl Mul<u64> for &ArithU256 {
    type Output = ArithU256;

    fn mul(self, rhs: u64) -> ArithU256 {
        self.clone() * rhs
    }
}

// Also let &ArithU256 * &ArithU256 => ArithU256
impl Mul<&ArithU256> for &ArithU256 {
    type Output = ArithU256;

    fn mul(self, rhs: &ArithU256) -> ArithU256 {
        self.clone() * rhs
    }
}

// &ArithU256 + &ArithU256 => ArithU256
impl Add<&ArithU256> for &ArithU256 {
    type Output = ArithU256;

    fn add(self, rhs: &ArithU256) -> ArithU256 {
        self.clone() + rhs
    }
}

// &ArithU256 - &ArithU256 => ArithU256
impl Sub<&ArithU256> for &ArithU256 {
    type Output = ArithU256;

    fn sub(self, rhs: &ArithU256) -> ArithU256 {
        self.clone() - rhs
    }
}

// ---------------------------------------------------------
// 5) Div ArithU256 / &ArithU256
//    So you can do: let q = r1.clone() / &r2;
// ---------------------------------------------------------
// &ArithU256 / &ArithU256 => ArithU256
impl Div<&ArithU256> for &ArithU256 {
    type Output = ArithU256;

    fn div(self, rhs: &ArithU256) -> ArithU256 {
        self.clone() / rhs
    }
}
