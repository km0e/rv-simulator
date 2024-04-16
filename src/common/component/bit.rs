use crate::common::abi::*;
pub enum Alloc {
    Out = 0,
}
pub enum Connect {
    In = 0,
}
#[derive(Clone)]
pub struct BitBuilder {
    pub interval: (u8, u8), //[]
    pub input: Option<PortRef>,
}
impl BitBuilder {
    pub fn new(interval: (u8, u8)) -> Self {
        Self {
            interval,
            input: None,
        }
    }
}
impl PortBuilder for BitBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, _id: Self::Alloc) -> PortRef {
        assert!(self.input.is_some(), "Bit input is not connected");
        PortRef::from(Bit::new(self.interval, self.input.clone().unwrap()))
    }
    fn connect(&mut self, pin: PortRef, _id: Self::Connect) {
        self.input = Some(pin);
    }
}

#[derive(Debug)]
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
