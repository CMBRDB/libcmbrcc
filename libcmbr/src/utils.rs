use std::ops::{BitAnd, Shl, Shr, Sub};

// Macro stolen from https://stackoverflow.com/a/62759540
macro_rules! def_enum {
    ($vis:vis $name:ident => $ty:ty {
        $($variant:ident => $val:expr),+
        $(,)?
    }) => {
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $name;

        impl $name {
            $(
                pub const $variant: $ty = $val;
            )+

            pub const VARIANTS: &'static [$ty] = &[$(Self::$variant),+];
        }
    };
}

pub(crate) use def_enum;

pub(crate) fn extract_bits_from_num<T>(number: T, num_bits: u32, start_position: u32) -> T
where
    T: Copy
        + BitAnd<Output = T>
        + Shl<u32, Output = T>
        + Shr<u32, Output = T>
        + From<u8>
        + Sub<Output = T>,
{
    let mask = (T::from(1u8) << num_bits) - T::from(1u8);
    return (number >> start_position) & mask;
}
