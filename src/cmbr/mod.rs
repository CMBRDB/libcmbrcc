mod u24_impl;
pub use u24_impl::*;

// Macro stolen from https://stackoverflow.com/a/62759540
macro_rules! def_enum {
    ($vis:vis $name:ident => $ty:ty {
        $($variant:ident => $val:expr),+
        $(,)?
    }) => {
        #[non_exhaustive]
        $vis struct $name;

        impl $name {
            $(
                pub const $variant: $ty = $val;
            )+

            pub const VARIANTS: &'static [$ty] = &[$(Self::$variant),+];
        }
    };
}

def_enum! (pub CmbrFlags => u8{
    FlagNone  => 0,
    FlagCheck   => 1 << 0,
    FlagMate    => 1 << 1,
    FlagCapture => 1 << 2,
    FlagNag     => 1 << 3, // Next 8 bits are a NAG index (https://w.wiki/AWUT)

    FlagPromotesBishop => (1 << 6) | 0b000000,
    FlagPromotesKnight => (1 << 6) | 0b010000,
    FlagPromotesRook   => (1 << 6) | 0b100000,
    FlagPromotesQueen  => (1 << 6) | 0b110000,

    FlagIsVariationPointer => 1 << 7 // First 8 bits are Index to the table of variations
});

pub type Cmbr = (u24, Option<u8>);

// fn san_to_cmbr(san: &[u8])
