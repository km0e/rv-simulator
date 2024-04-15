use crate::common::abi::*;
pub enum Alloc {
    Out = 0,
}
pub enum Connect {
    In = 0,
}
#[derive(Clone)]
pub struct BitBuilder {
    pub inner: PortShared<Bit>,
}
impl BitBuilder {
    pub fn new(interval: (u8, u8)) -> Self {
        Self {
            inner: PortShared::new(Bit {
                interval,
                data: None,
            }),
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
        self.inner.borrow_mut().data = Some(pin);
    }
}

#[derive(Debug)]
pub struct Bit {
    pub interval: (u8, u8), //[]
    pub data: Option<PortRef>,
}

impl Port for Bit {
    fn read(&self) -> u32 {
        let data = self.data.as_ref().unwrap().read();
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
