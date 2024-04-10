use super::{Builder, Component, ComponentRef, ComponentShared};
#[derive(Clone)]
pub struct BitBuilder {
    pub inner: ComponentShared<Bit>,
}
impl BitBuilder {
    pub fn new(interval: (u8, u8)) -> Self {
        Self {
            inner: ComponentShared::new(Bit {
                interval,
                data: None,
            }),
        }
    }
}
impl Builder for BitBuilder {
    fn alloc(&mut self, id: usize) -> ComponentRef {
        assert_eq!(id, 0);
        ComponentRef::from(self.inner.clone())
    }
    fn build(self) -> Option<super::ControlRef> {
        None
    }
    fn connect(&mut self, pin: ComponentRef, id: usize) {
        assert_eq!(id, 0);
        self.inner.borrow_mut().data = Some(pin);
    }
}

pub struct Bit {
    pub interval: (u8, u8), //[]
    pub data: Option<ComponentRef>,
}

impl Component for Bit {
    fn read(&self) -> u32 {
        let data = self.data.as_ref().unwrap().read();
        if self.interval.1 - self.interval.0 == 31 {
            return data;
        }
        let mask = (1 << (self.interval.1 - self.interval.0 + 1)) - 1;
        (data >> self.interval.0) & mask
    }
}
