use super::{lat::Lat, Builder, ComponentRef, ComponentShared, ControlRef};

#[derive(Default)]
pub struct ConstsBuilder {
    data: Vec<ComponentShared<Lat>>,
}
impl Builder for ConstsBuilder {
    fn connect(&mut self, pin: ComponentRef, id: usize) {
        unimplemented!()
    }
    fn alloc(&mut self, id: usize) -> ComponentRef {
        assert!(id < self.data.len());
        ComponentRef::from(self.data[id].clone())
    }
    fn build(self) -> Option<ControlRef> {
        None
    }
}
impl ConstsBuilder {
    pub fn push(&mut self, value: u32) {
        self.data.push(ComponentShared::new(Lat::new(value)));
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
