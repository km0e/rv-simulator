use crate::common::abi::*;
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
    fn alloc(&mut self, id: usize) -> PortRef {
        assert_eq!(id, 0);
        PortRef::from(self.inner.clone())
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        assert_eq!(id, 0);
        self.inner.borrow_mut().data = Some(pin);
    }
}

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
    pub use super::Bit as BitAlloc;
    pub use super::Bit as BitConnect;
    pub use super::BitBuilder;
}
