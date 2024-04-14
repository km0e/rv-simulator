// use crate::common::abi::*;
// use std::collections::HashMap;
// #[derive(Default)]
// pub struct Circuit {
//     id: usize,
//     components: HashMap<usize, ControlRef>,
// }
// impl Control for Circuit {
//     fn rasing_edge(&mut self) {
//         for (_, component) in self.components.iter_mut() {
//             component.rasing_edge();
//         }
//     }
//     fn falling_edge(&mut self) {
//         for (_, component) in self.components.iter_mut() {
//             component.falling_edge();
//         }
//     }
//     #[cfg(debug_assertions)]
//     fn debug(&self) -> String {
//         "Circuit".to_string()
//     }
// }
// impl Circuit {
//     pub fn new() -> Self {
//         Self {
//             id: 0,
//             components: HashMap::new(),
//         }
//     }

//     pub fn add(&mut self, component: ControlRef) -> usize {
//         let id = self.id;
//         self.id += 1;
//         self.components.insert(id, component);
//         id
//     }

//     pub fn get(&self, id: usize) -> ControlRef {
//         self.components[&id].clone()
//     }
// }
