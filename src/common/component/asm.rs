use crate::common::abi::*;
use std::{cell::RefCell, rc::Rc};
mod reg;
pub use reg::Alloc;
trait AsmPort {
    fn read(&self) -> String;
}
#[derive(Default)]
pub struct AsmPortShared<T: 'static + AsmPort>(Shared<T>);
impl<T: 'static + AsmPort> AsmPortShared<T> {
    pub fn new(asm: T) -> Self {
        Self(asm.into())
    }
    pub fn into_inner(self) -> Shared<T> {
        self.0
    }
}
// impl<T: 'static + AsmPort> ToShared<T> for AsmPortShared<T> {
//     fn to_shared(self) -> Shared<T> {
//         self.0
//     }
// }
impl<T: 'static + AsmPort> Clone for AsmPortShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
pub struct AsmPortRef(Rc<RefCell<dyn AsmPort>>);
impl AsmPortRef {
    pub fn read(&self) -> String {
        self.0.borrow().read()
    }
}
// impl<T: 'static + AsmPort> From<AsmPortShared<T>> for AsmPortRef {
//     fn from(shared: AsmPortShared<T>) -> Self {
//         Self(shared.to_shared().into())
//     }
// }

impl<T: 'static + AsmPort> From<Shared<T>> for AsmPortRef {
    fn from(asm: Shared<T>) -> Self {
        Self(asm.into_inner())
    }
}
impl<T: 'static + AsmPort + Control> From<ControlShared<T>> for AsmPortRef {
    fn from(shared: ControlShared<T>) -> Self {
        shared.into_shared().into()
    }
}
// impl<T: 'static + AsmPort> From<T> for AsmPortRef {
//     fn from(asm: T) -> Self {
//         Self(Rc::new(RefCell::new(asm)))
//     }
// }
impl Clone for AsmPortRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
pub trait AsmBuilder {
    fn asm_alloc(&self, id: usize) -> AsmPortRef;
    fn asm_connect(&mut self, pin: AsmPortRef, id: usize);
}
pub enum Connect {
    Address,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Address => 0,
        }
    }
}
#[derive(Default)]
pub struct AsmMemBuilder {
    pub inner: AsmPortShared<Asm>,
}
impl AsmMemBuilder {
    pub fn new(mem: Vec<String>) -> Self {
        Self {
            inner: AsmPortShared::new(Asm::new(mem)),
        }
    }
}
impl ControlBuilder for AsmMemBuilder {
    fn build(self) -> ControlRef {
        self.inner.into_inner().into()
    }
}
impl AsmBuilder for AsmMemBuilder {
    fn asm_alloc(&self, id: usize) -> AsmPortRef {
        assert_eq!(id, Connect::Address.into(), "AsmMemBuilder: invalid id");
        self.inner.clone().into_inner().into()
    }
    fn asm_connect(&mut self, _pin: AsmPortRef, _id: usize) {
        panic!("AsmMemBuilder: don't need to asm connect")
    }
}
impl PortBuilder for AsmMemBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, _id: Self::Alloc) -> PortRef {
        panic!("AsmMemBuilder: don't need to alloc")
    }
    fn connect(&mut self, pin: PortRef, id: Self::Connect) {
        match id {
            Self::Connect::Address => self.inner.0.borrow_mut().address = Some(pin),
            _ => panic!("AsmMemBuilder: invalid connect id"),
        }
    }
}
#[derive(Default)]
pub struct Asm {
    pub address: Option<PortRef>,
    pub mem: Vec<String>,
}
impl Asm {
    pub fn new(mem: Vec<String>) -> Self {
        Self { address: None, mem }
    }
}
impl Control for Asm {
    fn rasing_edge(&mut self) {}
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        "Asm".to_string()
    }
}
impl AsmPort for Asm {
    fn read(&self) -> String {
        let addr = self.address.as_ref().expect("address not connected").read() as usize;
        if addr < self.mem.len() * 4 {
            self.mem[addr / 4].clone()
        } else {
            "Invalid instruction".to_string()
        }
    }
}

pub mod build {
    pub use super::reg::Alloc as AsmRegAlloc;
    pub use super::reg::AsmRegBuilder;
    pub use super::reg::Connect as AsmRegConnect;
    pub use super::Alloc as AsmAlloc;
    pub use super::AsmBuilder;
    pub use super::AsmMemBuilder;
    pub use super::AsmPortRef;
    pub use super::AsmPortShared;
    pub use super::Connect as AsmConnect;
}
