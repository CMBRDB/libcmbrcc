use std::{collections::{BTreeSet, VecDeque}, ops::{BitAnd, Shl, Shr, Sub}};

// Macro stolen from https://stackoverflow.com/a/62759540
macro_rules! def_enum {
    ($(#[$outer:meta])* $vis:vis $name:ident => $ty:ty {
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

pub(crate) fn nth_prime_number<T>(n: u32) -> T
where T: std::cmp::Ord + std::clone::Clone + std::convert::From<u32>
{
    const PRIMES_LOOKUP: &[u32] = &[
        1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
        89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
        181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277,
        281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389,
        397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499,
        503, 509, 521, 523, 541,
    ];

    if n <= 100 {
        return T::from(PRIMES_LOOKUP[n as usize]);
    }

    let limit = if n < 6 {
        15 // Small value for small n
    } else {
        let log_n = (n as f64).ln();
        (n as f64 * (log_n + log_n.ln())).ceil() as usize
    };

    let mut isprime = VecDeque::from(vec![true; limit]);
    let mut prime = BTreeSet::new();

    isprime[0] = false;
    isprime[1] = false;

    for i in 2..limit {
        if isprime[i] {
            prime.insert(T::from(i as u32));

            for j in (i * i..limit).step_by(i) {
                isprime[j] = false;
            }
        }
    }

    prime.iter().nth(n as usize - 1).cloned().unwrap()
}