use crate::common::abi::*;
use std::{cell::RefCell, collections::BTreeSet, fmt::Debug, rc::Rc};
mod reg;
// pub use reg::Alloc;

pub enum Stage {
    None,
    Fetch,
    Decode,
    Execute,
    Memory,
    WriteBack,
}
impl ToString for Stage {
    fn to_string(&self) -> String {
        match self {
            Stage::None => "None".to_string(),
            Stage::Fetch => "Fetch".to_string(),
            Stage::Decode => "Decode".to_string(),
            Stage::Execute => "Execute".to_string(),
            Stage::Memory => "Memory".to_string(),
            Stage::WriteBack => "WriteBack".to_string(),
        }
    }
}
pub struct Inst {
    pub asm: String,
    pub stage: Stage,
}

trait AsmPort: Control + Debug {
    fn read(&self, len_hint: usize) -> Vec<Inst>;
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
impl<T: 'static + AsmPort> Clone for AsmPortShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[derive(Debug)]
pub struct AsmPortRef(Rc<RefCell<(dyn AsmPort)>>);
impl AsmPortRef {
    pub fn read(&self, len_hint: usize) -> Vec<Inst> {
        self.0.borrow().read(len_hint)
    }
    pub fn rasing_edge(&self) {
        self.0.borrow_mut().rasing_edge()
    }
    pub fn falling_edge(&self) {
        self.0.borrow_mut().falling_edge()
    }
}
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
impl Clone for AsmPortRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
pub trait AsmBuilder {
    fn build(self) -> AsmPortRef;
}
pub enum Connect {
    Address,
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
impl AsmBuilder for AsmMemBuilder {
    fn build(self) -> AsmPortRef {
        self.inner.into_inner().into()
    }
}
impl PortBuilder for AsmMemBuilder {
    type Alloc = ();
    type Connect = Connect;
    fn alloc(&mut self, _id: Self::Alloc) -> PortRef {
        panic!("AsmMemBuilder: don't need to alloc")
    }
    fn connect(&mut self, pin: PortRef, id: Self::Connect) {
        match id {
            Self::Connect::Address => self.inner.0.borrow_mut().address = Some(pin),
        }
    }
}
#[derive(Debug, Default)]
pub struct Asm {
    pub address: Option<PortRef>,
    pub address_cache: u32,
    pub set: BTreeSet<u32>,
    pub stages: Vec<Option<u32>>,
    pub mem: Vec<String>,
}
impl Asm {
    pub fn new(mem: Vec<String>) -> Self {
        let mut stages = vec![None; 5];
        stages[0] = Some(0);
        Self {
            address: None,
            address_cache: 0,
            set: BTreeSet::new(),
            stages,
            mem,
        }
    }
}
impl Control for Asm {
    fn rasing_edge(&mut self) {
        self.address_cache = self.address.as_ref().expect("address not connected").read() / 4;
    }
    fn falling_edge(&mut self) {
        let addr = self.address_cache as usize;
        if addr < self.mem.len() {
            if let Some(Some(stage)) = self.stages.last() {
                self.set.remove(stage);
            }
            self.stages.rotate_right(1);
            self.stages[0] = Some(self.address_cache);
            self.set.insert(self.address_cache);
        }
    }
    fn input(&self) -> Vec<(String, u32)> {
        unimplemented!("Asm: input")
    }
    fn output(&self) -> Vec<(String, u32)> {
        unimplemented!("Asm: output")
    }
}
// impl AsmControl for Asm {
//     fn asm(&self, addr: u32) -> String {
//         if addr < self.mem.len() as u32 {
//             self.mem[(addr / 4) as usize].clone()
//         } else {
//             "Invalid instruction".to_string()
//         }
//     }
// }
// pub trait AsmControl: Control {
//     fn asm(&self, addr: u32) -> String;
// }
impl AsmPort for Asm {
    fn read(&self, len_hint: usize) -> Vec<Inst> {
        let mut start = self.set.first().copied().unwrap_or(0);
        let mut end = self.set.last().copied().unwrap_or(0);
        if end - start < len_hint as u32 {
            let more = len_hint as u32 - (end - start);
            start = start.saturating_sub(more / 2);
            end = end.saturating_add(more - (more / 2));
            end = end.min(self.mem.len() as u32);
        }
        let mut res = self.mem[start as usize..end as usize]
            .iter()
            .map(|asm| Inst {
                asm: asm.clone(),
                stage: Stage::None,
            })
            .collect::<Vec<_>>();
        self.stages
            .iter()
            .zip([
                Stage::Fetch,
                Stage::Decode,
                Stage::Execute,
                Stage::Memory,
                Stage::WriteBack,
            ])
            .for_each(|(stage, stage_name)| {
                if let Some(addr) = stage {
                    if *addr >= start && *addr < end {
                        res[*addr as usize - start as usize].stage = stage_name;
                    }
                }
            });
        res
    }
}

pub mod build {
    // pub use super::reg::Alloc as AsmRegAlloc;
    // pub use super::reg::AsmRegBuilder;
    // pub use super::reg::Connect as AsmRegConnect;
    // pub use super::Alloc as AsmAlloc;
    pub use super::AsmBuilder;
    pub use super::AsmMemBuilder;
    pub use super::AsmPortRef;

    pub use super::Connect as AsmConnect;
}
