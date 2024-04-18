use crate::common::abi::*;

#[derive(Default, Debug)]
pub struct Bomb {}

impl Port for Bomb {
    fn read(&self) -> u32 {
        unreachable!("Bomb")
    }
}
pub fn bomb() -> Shared<Bomb> {
    Shared::from(Bomb::default())
}
pub mod build {
    pub use super::bomb;
}
