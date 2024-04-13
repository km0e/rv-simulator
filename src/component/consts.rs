use super::{lat::Lat, Builder, ControlRef, PortRef, PortShared};

#[derive(Default)]
pub struct ConstsBuilder {
    data: Vec<PortShared<Lat>>,
}
impl Builder for ConstsBuilder {
    fn connect(&mut self, pin: PortRef, id: usize) {
        unimplemented!()
    }
    fn alloc(&mut self, id: usize) -> PortRef {
        assert!(id < self.data.len());
        PortRef::from(self.data[id].clone())
    }
    fn build(self) -> Option<ControlRef> {
        None
    }
}
impl ConstsBuilder {
    pub fn push(&mut self, value: u32) {
        self.data.push(PortShared::new(Lat::new(value)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consts() {
        let mut consts = ConstsBuilder::default();
        consts.push(1);
        consts.push(2);
        assert_eq!(consts.alloc(0).read(), 1);
        assert_eq!(consts.alloc(1).read(), 2);
    }
}
