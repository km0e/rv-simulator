use crate::common::abi::*;
use crate::common::build::*;
pub enum Alloc {
    Out(usize),
}
#[allow(dead_code)]
pub enum Connect {}
#[derive(Default)]
pub struct ConstsBuilder {
    data: Vec<PortShared<Lat>>,
}
impl PortBuilder for ConstsBuilder {
    type Connect = ();
    type Alloc = Alloc;
    fn connect(&mut self, _pin: PortRef, _id: Self::Connect) {
        unreachable!("ConstsBuilder does not have any input");
    }
    fn alloc(&mut self, id: Self::Alloc) -> PortRef {
        let ps = match id {
            Alloc::Out(id) => self.data[id].clone(),
        };
        ps.into()
    }
}
impl ConstsBuilder {
    pub fn push(&mut self, value: u32) {
        self.data.push(PortShared::new(Lat::new(value)));
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
        consts.push(1);
        consts.push(2);
        assert_eq!(consts.alloc(Alloc::Out(0)).read(), 1);
        assert_eq!(consts.alloc(Alloc::Out(1)).read(), 2);
    }
}
