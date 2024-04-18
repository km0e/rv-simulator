use crate::common::abi::*;
use crate::common::build::*;
pub enum Alloc {
    Out = 0,
}
pub enum Connect {
    In = 0,
}

#[derive(Clone, Debug)]
pub struct BitBuilder {
    pub inner: PortShared<Bit>,
}
impl BitBuilder {
    pub fn new(interval: (u8, u8)) -> Self {
        Self {
            inner: PortShared::new(Bit::new(interval, bomb().into())),
        }
    }
}
impl PortBuilder for BitBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, _id: Self::Alloc) -> PortRef {
        PortRef::from(self.inner.clone())
    }
    fn connect(&mut self, pin: PortRef, _id: Self::Connect) {
        self.inner.borrow_mut().input = pin;
    }
}

#[derive(Debug, Clone)]
pub struct Bit {
    pub interval: (u8, u8), //[]
    pub input: PortRef,
}
impl Bit {
    pub fn new(interval: (u8, u8), input: PortRef) -> Self {
        Self { interval, input }
    }
}
impl Port for Bit {
    fn read(&self) -> u32 {
        let data = self.input.read();
        if self.interval.1 - self.interval.0 == 31 {
            return data;
        }
        let mask = (1 << (self.interval.1 - self.interval.0 + 1)) - 1;
        (data >> self.interval.0) & mask
    }
}
pub mod build {
    pub use super::Alloc as BitAlloc;
    pub use super::BitBuilder;
    pub use super::Connect as BitConnect;
}
