#![allow(non_upper_case_globals)]
#![feature(test)]

// TODO: Experiment with different allocators
// Since our program is memory-usage intensive, different allocators may provide performance speedups and use less memory

// NOTE: With TCMAlloc the program is just slightly faster (by like 400ns/iter)
#[cfg(all(not(target_env = "msvc"), feature = "tcmalloc"))]
use tcmalloc::TCMalloc;

#[cfg(all(not(target_env = "msvc"), feature = "tcmalloc"))]
#[global_allocator]
static ALLOCATOR: TCMalloc = TCMalloc;

pub mod cmbr;
pub mod pgn;
// mod tests;
mod utils;

pub use shakmaty::Chess as ChessBoard;
