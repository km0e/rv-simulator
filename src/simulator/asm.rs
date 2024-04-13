use std::{cell::RefCell, rc::Rc};

use crate::{
    component::{ControlRef, ControlShared, Port, PortRef},
    Builder, Control,
};
pub enum Alloc {
    Output,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::Output => 0,
        }
    }
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
pub struct AsmBuilder {
    pub inner: ControlShared<Asm>,
}
impl AsmBuilder {
    pub fn new(mem: Vec<String>) -> Self {
        Self {
            inner: ControlShared::new(Asm::new(mem)),
        }
    }
    pub fn alloc_asm(&mut self) -> Rc<RefCell<AsmReg>> {
        self.inner.borrow_mut().output.clone().unwrap()
    }
}
impl Builder for AsmBuilder {
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.inner.borrow_mut().address = Some(pin),
            _ => panic!("Invalid id"),
        }
    }
    fn alloc(&mut self, _: usize) -> PortRef {
        unreachable!("AsmBuilder does not support alloc")
    }
    fn build(self) -> Option<ControlRef> {
        Some(ControlRef::from(self.inner.clone()))
    }
}
#[derive(Default)]
pub struct Asm {
    pub address: Option<PortRef>,
    pub address_cache: usize,
    pub output: Option<Rc<RefCell<AsmReg>>>,
    pub mem: Vec<String>,
}
impl Asm {
    pub fn new(mem: Vec<String>) -> Self {
        Self {
            address: None,
            address_cache: 0,
            output: Some(Rc::new(RefCell::new(AsmReg::default()))),
            mem,
        }
    }
}
impl Control for Asm {
    fn rasing_edge(&mut self) {
        self.address_cache = self.address.as_ref().expect("address not connected").read() as usize;
    }
    fn falling_edge(&mut self) {
        if let Some(output) = self.output.as_ref() {
            let mut output = output.borrow_mut();
            if self.address_cache < self.mem.len() * 4 {
                output.inst = self.mem[self.address_cache / 4].clone();
            } else {
                output.inst = "Invalid instruction".to_string();
            }
        }
    }
    fn debug(&self) -> String {
        self.output.as_ref().unwrap().borrow().inst.to_string()
    }
}
#[derive(Default)]
pub struct AsmRegBuilder {
    pub inner: Rc<RefCell<AsmReg>>,
}
impl AsmRegBuilder {
    pub fn new(prev: Rc<RefCell<AsmReg>>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(AsmReg {
                prev: Some(prev),
                tmp: String::new(),
                inst: String::new(),
            })),
        }
    }
    pub fn alloc_asm(&self) -> Rc<RefCell<AsmReg>> {
        self.inner.clone()
    }
}
#[derive(Default)]
pub struct AsmReg {
    pub prev: Option<Rc<RefCell<AsmReg>>>,
    pub tmp: String,
    pub inst: String,
}
impl Control for AsmReg {
    fn rasing_edge(&mut self) {
        self.tmp = self.prev.as_ref().unwrap().borrow().inst.clone();
    }
    fn falling_edge(&mut self) {
        self.inst = self.tmp.clone();
    }
    fn debug(&self) -> String {
        self.inst.clone()
    }
}
