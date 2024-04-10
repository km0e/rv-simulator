use std::collections::HashMap;

use crate::component::{self, Component, Control};
#[derive(Default)]
pub struct Circuit {
    id: usize,
    components: HashMap<usize, component::ControlRef>,
}
impl Control for Circuit {
    fn rasing_edge(&mut self) {
        for (_, component) in self.components.iter_mut() {
            component.rasing_edge();
        }
    }
    fn falling_edge(&mut self) {
        for (_, component) in self.components.iter_mut() {
            component.falling_edge();
        }
    }
}
impl Circuit {
    pub fn new() -> Self {
        Self {
            id: 0,
            components: HashMap::new(),
        }
    }

    pub fn add(&mut self, component: component::ControlRef) -> usize {
        let id = self.id;
        self.id += 1;
        self.components.insert(id, component);
        id
    }

    pub fn get(&self, id: usize) -> component::ControlRef {
        self.components[&id].clone()
    }
}
