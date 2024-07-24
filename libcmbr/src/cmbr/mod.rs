pub mod pgntocmbr;
pub mod santocmbrmv;
pub mod structs;
mod tests;
mod u24_impl;

pub use santocmbrmv::*;
pub use structs::*;
pub use u24_impl::*;

impl CmbrFile {
    pub fn serialize(&self) -> Vec<u8> {
        return bitcode::serialize(&self).unwrap();
    }
}