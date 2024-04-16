// use crate::common::abi::*;

// use super::{AsmBuilder, AsmPort, AsmPortRef};
// pub enum Alloc {
//     Out,
// }
// impl From<Alloc> for usize {
//     fn from(alloc: Alloc) -> usize {
//         match alloc {
//             Alloc::Out => 0,
//         }
//     }
// }
// pub enum Connect {
//     In,
// }
// impl From<Connect> for usize {
//     fn from(alloc: Connect) -> usize {
//         match alloc {
//             Connect::In => 0,
//         }
//     }
// }

// #[derive(Default)]
// pub struct AsmRegBuilder {
//     pub inner: Option<ControlShared<AsmReg>>,
// }
// impl ControlBuilder for AsmRegBuilder {
//     fn build(self) -> ControlRef {
//         self.inner.unwrap().into()
//     }
// }
// impl AsmBuilder for AsmRegBuilder {
//     fn asm_alloc(&self, id: usize) -> AsmPortRef {
//         assert_eq!(id, Alloc::Out.into(), "AsmRegBuilder: invalid asm alloc id");
//         if let Some(inner) = &self.inner {
//             inner.clone().into()
//         } else {
//             panic!("AsmRegBuilder: asm alloc before asm connect")
//         }
//     }
//     fn asm_connect(&mut self, pin: AsmPortRef, id: usize) {
//         assert_eq!(
//             id,
//             Connect::In.into(),
//             "AsmRegBuilder: invalid asm connect id"
//         );
//         self.inner = Some(ControlShared::new(AsmReg::new(pin)));
//     }
// }
// #[derive(Debug)]
// pub struct AsmReg {
//     pub prev: AsmPortRef,
//     pub tmp: String,
//     pub inst: String,
// }
// impl AsmReg {
//     pub fn new(ap: impl Into<AsmPortRef>) -> Self {
//         Self {
//             prev: ap.into(),
//             tmp: "".to_string(),
//             inst: "".to_string(),
//         }
//     }
// }
// impl Control for AsmReg {
//     fn rasing_edge(&mut self) {
//         self.tmp = self.prev.read();
//     }
//     fn falling_edge(&mut self) {
//         self.inst = self.tmp.clone();
//     }
//     fn input(&self) -> Vec<(String, u32)> {
//         unimplemented!("AsmReg: input")
//     }
//     fn output(&self) -> Vec<(String, u32)> {
//         unimplemented!("AsmReg: output")
//     }
// }

// impl AsmPort for AsmReg {
//     fn read(&self) -> String {
//         self.inst.clone()
//     }
// }
