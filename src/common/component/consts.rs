use std::collections::HashMap;

use crate::common::abi::*;
pub enum Alloc {
    Out(u32),
}
#[allow(dead_code)]
pub enum Connect {}
#[derive(Debug)]
pub struct Consts {
    data: u32,
}
impl Port for Consts {
    fn read(&self) -> u32 {
        self.data
    }
}
#[derive(Default)]
pub struct ConstsBuilder {
    set: HashMap<u32, PortRef>,
}
impl PortBuilder for ConstsBuilder {
    type Connect = ();
    type Alloc = Alloc;
    fn connect(&mut self, _pin: PortRef, _id: Self::Connect) {
        unreachable!("ConstsBuilder does not have any input");
    }
    fn alloc(&mut self, id: Self::Alloc) -> PortRef {
        let Alloc::Out(data) = id;
        self.set
            .entry(data)
            .or_insert_with(|| Shared::from(Consts { data }).into())
            .clone()
    }
}
pub mod build {
    pub use super::Alloc as ConstsAlloc;

    pub use super::ConstsBuilder;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consts() {
        let mut consts = ConstsBuilder::default();
        let out0 = consts.alloc(Alloc::Out(0));
        let out1 = consts.alloc(Alloc::Out(1));
        assert_eq!(out0.read(), 0);
        assert_eq!(out1.read(), 1);
    }
}
