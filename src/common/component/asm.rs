use crate::common::abi::*;
use crate::common::build::*;
use std::collections::BTreeMap;
use std::{cell::RefCell, collections::BTreeSet, fmt::Debug, rc::Rc};
mod reg;
// pub use reg::Alloc;
#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub addr: usize,
}
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
            Stage::None => "".to_string(),
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
    pub fn into_shared(self) -> Shared<T> {
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
impl<T: 'static + AsmPort + Control> From<AsmPortShared<T>> for AsmPortRef {
    fn from(shared: AsmPortShared<T>) -> Self {
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
    IfEn,
    IdEn,
    IdClr,
    ExClr,
}
#[derive(Debug)]
pub struct AsmMemory {
    pub func: Vec<Func>,
    pub data: BTreeMap<usize, String>,
}
impl AsmMemory {
    // Path: src/common/asm.rs
    // resolve the asm string to a AsmMemory struct
    // format:
    // #empty line
    // 0x00000000 <_start>:
    //    0:	00000517          	auipc	t0,0x0
    //    4:	00052503          	lw	t0,0(t0)
    // #empty line
    // 0x00000008 <main>:
    //    8:	00000517          	auipc	t0,0x0
    pub fn with_asm(asm: String) -> AsmMemory {
        let mut funcs = vec![];
        let mut data = BTreeMap::new();
        let mut head = false;
        let mut func: Func = Func {
            name: "".to_string(),
            addr: 0,
        };
        for line in asm.lines() {
            if line.is_empty() {
                head = true;
                continue;
            }
            if head {
                let (faddr, fname) = line.split_once(" ").unwrap();
                func.addr = usize::from_str_radix(faddr, 16).unwrap();
                func.name = fname[1..fname.len() - 2].to_string();
                funcs.push(func.clone());
                head = false;
                continue;
            }
            let line = line.trim();
            let addr = line.split_once(':').unwrap().0;
            let addr = usize::from_str_radix(addr, 16).unwrap();
            data.insert(addr, line.to_string());
        }
        AsmMemory { func: funcs, data }
    }
}

pub struct AsmMemBuilder {
    pub addr: PortRef,
    pub if_en: PortRef,
    pub id_en: PortRef,
    pub id_clr: PortRef,
    pub ex_clr: PortRef,
    pub entry: usize,
    pub mem: AsmMemory,
}
impl AsmMemBuilder {
    pub fn new(entry: usize, asm: String) -> Self {
        Self {
            addr: bomb().into(),
            if_en: bomb().into(),
            id_en: bomb().into(),
            id_clr: bomb().into(),
            ex_clr: bomb().into(),
            entry,
            mem: AsmMemory::with_asm(asm),
        }
    }
}
impl AsmBuilder for AsmMemBuilder {
    fn build(self) -> AsmPortRef {
        AsmPortShared::new(Asm::new(
            self.addr,
            self.if_en,
            self.id_en,
            self.id_clr,
            self.ex_clr,
            self.entry,
            self.mem,
        ))
        .into()
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
            Self::Connect::Address => self.addr = pin,
            Self::Connect::IfEn => self.if_en = pin,
            Self::Connect::IdEn => self.id_en = pin,
            Self::Connect::IdClr => self.id_clr = pin,
            Self::Connect::ExClr => self.ex_clr = pin,
        }
    }
}
#[derive(Debug)]
pub struct Asm {
    pub addr: PortRef,
    pub addr_cache: u32,
    pub if_en: PortRef,
    pub if_en_cache: u32,
    pub id_en: PortRef,
    pub id_en_cache: u32,
    pub id_clr: PortRef,
    pub id_clr_cache: u32,
    pub ex_clr: PortRef,
    pub ex_clr_cache: u32,
    pub set: BTreeSet<u32>,
    pub stages: Vec<Option<u32>>,
    pub entry: usize,
    pub mem: AsmMemory,
}
impl Asm {
    pub fn new(
        addr: PortRef,
        if_en: PortRef,
        id_en: PortRef,
        id_clr: PortRef,
        ex_clr: PortRef,
        entry: usize,
        mem: AsmMemory,
    ) -> Self {
        let mut stages = vec![None; 5];
        stages[0] = Some(entry as u32);
        Self {
            addr,
            addr_cache: 0,
            if_en,
            if_en_cache: 0,
            id_en,
            id_en_cache: 0,
            id_clr,
            id_clr_cache: 0,
            ex_clr,
            ex_clr_cache: 0,
            set: BTreeSet::from([entry as u32]),
            stages,
            entry,
            mem,
        }
    }
}
impl Control for Asm {
    fn rasing_edge(&mut self) {
        self.addr_cache = self.addr.read();
        self.if_en_cache = self.if_en.read();
        self.id_en_cache = self.id_en.read();
        self.id_clr_cache = self.id_clr.read();
        self.ex_clr_cache = self.ex_clr.read();
    }
    fn falling_edge(&mut self) {
        if let Some(Some(stage)) = self.stages.last() {
            self.set.remove(stage);
        }
        self.stages.rotate_right(1);
        if self.if_en_cache != 1 {
            self.stages[0] = self.stages[1];
        } else {
            self.stages[0] = Some(self.addr_cache);
            self.set.insert(self.addr_cache);
        }
        if self.id_clr_cache != 0 {
            self.stages[1] = None;
        } else if self.id_en_cache != 1 {
            self.stages[1] = self.stages[2];
        }
        if self.ex_clr_cache != 0 {
            self.stages[2] = None;
        }
    }
}
impl AsmPort for Asm {
    fn read(&self, mut len_hint: usize) -> Vec<Inst> {
        if self.set.is_empty() {
            return vec![];
        }
        len_hint *= 4;
        let mut start = self.set.first().copied().unwrap() as usize;
        let mut end = self.set.last().copied().unwrap() as usize;
        if end - start < len_hint {
            let more = len_hint - (end - start);
            start = start.saturating_sub(more / 2);
            end = start + len_hint;
        }
        let min = *self.mem.data.first_key_value().unwrap().0;
        let max = *self.mem.data.last_key_value().unwrap().0;
        if start < min {
            end += min - start;
            start = min;
        } else if end > max {
            let step = (end - max).min(start - min);
            start -= step;
            end -= step;
        }
        let mut res = self
            .mem
            .data
            .range(start..=end)
            .map(|(_, asm)| Inst {
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
                    if *addr >= start as u32 && *addr < end as u32 {
                        res[((*addr) as usize - start) / 4].stage = stage_name;
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
