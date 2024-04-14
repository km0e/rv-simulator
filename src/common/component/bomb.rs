use crate::common::abi::*;

#[derive(Default)]
pub struct Bomb {}

impl Port for Bomb {
    fn read(&self) -> u32 {
        unimplemented!()
    }
}

pub mod build {
    pub use super::Bomb;
}
