use super::Builder;
use super::Component;
use super::ComponentRef;
use super::ComponentShared;
use super::ControlRef;

#[derive(Default)]
pub struct AddBuilder {
    pub add: ComponentShared<Add>,
}
impl AddBuilder {
    pub fn new() -> Self {
        Self {
            add: ComponentShared::new(Add::default()),
        }
    }
}

impl Builder for AddBuilder {
    fn connect(&mut self, pin: ComponentRef, _: usize) {
        self.add.borrow_mut().input.push(pin);
    }
    fn alloc(&mut self, _id: usize) -> ComponentRef {
        super::ComponentRef::from(self.add.clone())
    }
    fn build(self) -> Option<ControlRef> {
        None
    }
}

#[derive(Default)]
pub struct Add {
    pub input: Vec<ComponentRef>,
}

impl Component for Add {
    fn read(&self) -> u32 {
        self.input.iter().map(|x| x.read()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::consts::ConstsBuilder;

    #[test]
    fn test_add() {
        let mut tb = AddBuilder::default();
        let mut constant = ConstsBuilder::default();
        constant.push(1);
        constant.push(2);
        let t = tb.alloc(0);
        tb.connect(constant.alloc(0), 0);
        tb.connect(constant.alloc(1), 1);
        assert_eq!(t.read(), 3);
        assert_eq!(t.read(), 3);
    }
}
