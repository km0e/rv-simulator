use crate::common::abi::*;

#[derive(Default, Debug)]
pub struct Bomb {}

impl Port for Bomb {
    fn read(&self) -> u32 {
        unreachable!("Bomb")
    }
}

pub mod build {
    pub use super::Bomb;
}
