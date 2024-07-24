#![allow(non_upper_case_globals)]
#![feature(test, map_try_insert)]

use cfg_if::cfg_if;

// TODO(#27): Add multithreading support
// TODO(#26): Experiment with different allocators
// Since our program is memory-usage intensive, different allocators may provide performance speedups and use less memory

// NOTE: With TCMAlloc the program is just slightly faster (by like 400ns/iter)
cfg_if! {
    if #[cfg(all(not(target_env = "msvc"), feature = "tcmalloc"))] {
        use tcmalloc::TCMalloc;

        #[global_allocator]
        static ALLOCATOR: TCMalloc = TCMalloc;
    }
}

pub mod cmbr;
pub mod error;
pub mod pgn;
pub mod position;
mod utils;

pub use shakmaty::Chess as ChessBoard;
