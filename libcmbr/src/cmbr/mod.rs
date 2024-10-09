pub mod pgntocmbr;
pub mod santocmbrmv;
pub mod structs;
mod tests;
mod u24_impl;

pub use santocmbrmv::*;
pub use structs::*;
pub use u24_impl::*;

use std::error::Error;

impl CmbrFile {
    pub fn dump_to_db(&self, _db_name: &str) -> Result<(), Box<dyn Error>> {
        todo!();
    }
}
