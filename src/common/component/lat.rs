use crate::common::abi::*;

#[derive(Default, Debug)]
pub struct Lat {
    pub data: u32,
}
impl Lat {
    pub fn new(value: u32) -> Self {
        Self { data: value }
    }
}
impl Port for Lat {
    fn read(&self) -> u32 {
        self.data
    }
}
pub mod build {
    pub use super::Lat;
}
