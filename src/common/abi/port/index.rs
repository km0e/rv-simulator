// use crate::abi::utils::ToShared;
use crate::common::abi::port::simple::{Port, PortRef, PortShared};
use crate::common::abi::utils::Shared;
pub trait IndexPort {
    fn read(&self, index: usize) -> u32;
}

pub struct IndexPortShared<T: IndexPort>(Shared<T>);
impl<T: IndexPort> IndexPortShared<T> {
    pub fn new(component: T) -> Self {
        Self(Shared::from(component))
    }
    pub fn borrow(&self) -> &T {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> &mut T {
        self.0.borrow_mut()
    }
}
// impl<T: IndexPort> ToShared<T> for IndexPortShared<T> {
//     fn to_shared(self) -> Shared<T> {
//         self.0
//     }
// }
impl<T: IndexPort> Clone for IndexPortShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
pub struct IndexPortRef<T: IndexPort>(Shared<T>);
impl<T: IndexPort> IndexPortRef<T> {
    pub fn read(&self, index: usize) -> u32 {
        self.0.borrow().read(index)
    }
}
impl<T: IndexPort> From<IndexPortShared<T>> for IndexPortRef<T> {
    fn from(shared: IndexPortShared<T>) -> Self {
        Self(shared.to_shared())
    }
}
impl<T: IndexPort> Clone for IndexPortRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct IndexPortWrapper<T: 'static + IndexPort>(Shared<T>, usize);
impl<T: IndexPort> IndexPortWrapper<T> {
    pub fn new(component: T, idx: usize) -> Self {
        Self(Shared::from(component), idx)
    }
}
impl<T: 'static + IndexPort> Port for IndexPortWrapper<T> {
    fn read(&self) -> u32 {
        self.0.borrow().read(self.1)
    }
}
impl<T: 'static + IndexPort> From<IndexPortWrapper<T>> for PortRef {
    fn from(shared: IndexPortWrapper<T>) -> Self {
        PortShared::new(shared).into()
    }
}
pub trait IndexPortBuilder<T: IndexPort> {
    fn index_connect(&mut self, pin: IndexPortRef<T>, id: usize);
    fn index_alloc(&mut self, id: usize) -> IndexPortRef<T>;
}
