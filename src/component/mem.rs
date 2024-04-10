use super::{lat::Lat, Builder, ComponentRef, ComponentShared, Control, ControlRef, ControlShared};
#[derive(Default)]
pub struct MemBuilder {
    pub inner: ControlShared<Mem>,
}
impl MemBuilder {
    pub fn new(memory: Vec<u8>) -> Self {
        Self {
            inner: ControlShared::new(Mem {
                data: memory,
                ..Default::default()
            }),
        }
    }
}
impl Builder for MemBuilder {
    // Connect the address and input pin
    fn connect(&mut self, pin: ComponentRef, id: usize) {
        match id {
            0 => self.inner.borrow_mut().address = Some(pin),
            1 => self.inner.borrow_mut().input = Some(pin),
            2 => self.inner.borrow_mut().write = Some(pin),
            _ => panic!("Invalid id"),
        }
    }
    // alloc the id for the memory
    // 0 for address
    // 1 for input
    fn alloc(&mut self, _: usize) -> ComponentRef {
        ComponentRef::from(self.inner.borrow().output.clone())
    }
    fn build(self) -> Option<ControlRef> {
        Some(ControlRef::from(self.inner.clone()))
    }
}
#[derive(Default)]
pub struct Mem {
    pub id: usize,
    pub data: Vec<u8>,
    pub input: Option<ComponentRef>,
    pub write: Option<ComponentRef>,
    pub address: Option<ComponentRef>,
    pub address_cache: usize,
    pub output: ComponentShared<Lat>,
}
impl Control for Mem {
    fn rasing_edge(&mut self) {
        if self.write.as_ref().unwrap().read() == 1 {
            if let Some(address) = self.address.as_ref() {
                self.address_cache = address.read() as usize;
                let arr = self.data.as_mut_slice();
                let value = self.input.as_ref().unwrap().read();
                arr[self.address_cache] = (value & 0xff) as u8;
                arr[self.address_cache + 1] = ((value >> 8) & 0xff) as u8;
                arr[self.address_cache + 2] = ((value >> 16) & 0xff) as u8;
                arr[self.address_cache + 3] = ((value >> 24) & 0xff) as u8;
            }
        }
    }
    fn falling_edge(&mut self) {
        let arr = self.data.as_slice();
        self.output.borrow_mut().data = u32::from_ne_bytes([
            arr[self.address_cache],
            arr[self.address_cache + 1],
            arr[self.address_cache + 2],
            arr[self.address_cache + 3],
        ]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::build::AddBuilder;
    use crate::component::build::RegBuilder;
    use crate::component::consts::ConstsBuilder;

    #[test]
    fn test_mem() {
        let mut tb = MemBuilder::new(b"12345678".to_vec());
        let mut constant = ConstsBuilder::default();
        constant.push(1);
        constant.push(1);
        let mut ab = AddBuilder::default();
        let add = ab.alloc(0);
        ab.connect(constant.alloc(0), 0);
        let mut rb = RegBuilder::new(0);
        rb.connect(ab.alloc(0), 0);
        ab.connect(rb.alloc(0), 0);
        tb.connect(rb.alloc(0), 0);
        tb.connect(constant.alloc(1), 1);
        tb.connect(rb.alloc(0), 2);
        let t = tb.alloc(0);
        let tc = tb.build().unwrap();
        let rc = rb.build().unwrap();
        tc.rasing_edge();
        // println!("{:#X}", t.borrow().read(0).unwrap());
        assert_eq!(t.read(), u32::from_ne_bytes([0, 0, 0, 0]));
        tc.falling_edge();
        assert_eq!(t.read(), u32::from_ne_bytes([0x31, 0x32, 0x33, 0x34]));
        rc.rasing_edge();
        assert_eq!(add.read(), 1);
        rc.falling_edge();
        assert_eq!(add.read(), 2);
        tc.rasing_edge();
        tc.falling_edge();
        assert_eq!(t.read(), u32::from_ne_bytes([1, 0, 0, 0]));
    }
    #[test]
    #[should_panic]
    fn test_mem_panic() {
        let mut tb = MemBuilder::default();
        let mut constant = ConstsBuilder::default();
        constant.push(1);
        constant.push(2);
        constant.push(3);
        tb.connect(constant.alloc(0), 0);
        tb.connect(constant.alloc(1), 1);
        tb.connect(constant.alloc(2), 2);
        tb.connect(constant.alloc(2), 3);
    }
}
