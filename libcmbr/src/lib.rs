#![allow(non_upper_case_globals, clippy::needless_return)]
#![feature(test, map_try_insert, stmt_expr_attributes)]

use cfg_if::cfg_if;

// TODO(#27): Add multithreading support
// TODO(#26): Experiment with different allocators
// Since our program is memory-usage intensive, different allocators may provide performance speedups and use less memory

cfg_if! {
    if #[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))] {
        use tikv_jemallocator::Jemalloc;

        #[global_allocator]
        static ALLOCATOR: Jemalloc = Jemalloc;
    }
}

pub mod cmbr;
pub mod error;
pub mod pgn;
mod utils;

pub use shakmaty::Chess as ChessBoard;
