use crate::utils::extract_bits_from_num;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};
use std::ops::{
    AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, DivAssign, MulAssign, RemAssign, ShlAssign,
    ShrAssign, SubAssign,
};

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
/// Unsigned 24bit integer
pub struct u24([u8; 3]);

impl u24 {
    pub fn to_u32(self) -> u32 {
        let u24([a, b, c]) = self;

        return u32::from_le_bytes([a, b, c, 0]);
    }

    pub fn from_u32(n: u32) -> Self {
        let [a, b, c, _d] = n.to_le_bytes();

        #[cfg(feature = "safe_u24")]
        debug_assert!(_d == 0);
        return u24([a, b, c]);
    }
}

macro_rules! impl_op_rhs {
    ($trait:ident, $fn:ident, $op:tt, $type:ty) => {
        impl $trait for $type {
            type Output = Self;

            #[inline(always)]
            fn $fn(self, rhs: Self) -> Self {
                return Self::from_u32(self.to_u32() $op rhs.to_u32());
            }
        }
    };
}

macro_rules! impl_op_rhs_assign {
    ($trait:ident, $fn:ident, $op:tt, $type:ty) => {
        impl $trait for $type {
            #[inline(always)]
            fn $fn(&mut self, rhs: Self) {
                *self = Self::from_u32(self.to_u32() $op rhs.to_u32())
            }
        }
    };
}

macro_rules! impl_op_single {
    ($trait:ident, $fn:ident, $op:tt, $type:ty) => {
        impl $trait for $type {
            type Output = Self;

            #[inline(always)]
            fn $fn(self) -> Self {
                return Self::from_u32($op self.to_u32());
            }
        }
    };
}

impl_op_rhs!(Add, add, +, u24);
impl_op_rhs!(Sub, sub, -, u24);
impl_op_rhs!(Mul, mul, *, u24);
impl_op_rhs!(Div, div, /, u24);
impl_op_rhs!(Rem, rem, %, u24);
impl_op_rhs!(Shr, shr, >>, u24);
impl_op_rhs!(Shl, shl, <<, u24);
impl_op_rhs!(BitAnd, bitand, &, u24);
impl_op_rhs!(BitOr,  bitor,  |, u24);
impl_op_rhs!(BitXor, bitxor, ^, u24);

impl_op_single!(Not, not, !, u24);

impl_op_rhs_assign!(AddAssign, add_assign, +, u24);
impl_op_rhs_assign!(SubAssign, sub_assign, -, u24);
impl_op_rhs_assign!(MulAssign, mul_assign, *, u24);
impl_op_rhs_assign!(DivAssign, div_assign, /, u24);
impl_op_rhs_assign!(RemAssign, rem_assign, %, u24);
impl_op_rhs_assign!(ShrAssign, shr_assign, >>, u24);
impl_op_rhs_assign!(ShlAssign, shl_assign, <<, u24);
impl_op_rhs_assign!(BitAndAssign, bitand_assign, &, u24);
impl_op_rhs_assign!(BitOrAssign,  bitor_assign,  |, u24);
impl_op_rhs_assign!(BitXorAssign, bitxor_assign, ^, u24);

impl u24 {
    #[inline(always)]
    pub fn count_ones(self) -> u32 {
        return self.to_u32().count_ones();
    }

    #[inline(always)]
    pub fn count_zeros(self) -> u32 {
        return 24 - self.count_ones();
    }

    #[inline(always)]
    pub fn trailing_zeroes(self) -> u32 {
        return self.to_u32().trailing_zeros() - 8;
    }
}

impl From<u32> for u24 {
    fn from(v: u32) -> Self {
        return u24::from_u32(v);
    }
}

impl From<u24> for u32 {
    fn from(v: u24) -> Self {
        return v.to_u32();
    }
}

impl fmt::Debug for u24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.to_u32());
    }
}

impl fmt::Display for u24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.to_u32());
    }
}

impl fmt::Binary for u24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:#026b}", self.to_u32());
    }
}

impl fmt::LowerExp for u24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_u32();
        let e = extract_bits_from_num::<u32>;
        return write!(
            f,
            "{:06b} {:06b} {:04b} {:08b}",
            e(s, 6, 18),
            e(s, 6, 12),
            e(s, 4, 8),
            e(s, 8, 0)
        );
    }
}
