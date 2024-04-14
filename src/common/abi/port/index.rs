use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::super::utils::Shared;
pub trait IndexPort {
    fn read(&self, index: usize) -> u32;
}
pub struct IndexPortShared<T: 'static + IndexPort>(Shared<T>);
impl<T: IndexPort> IndexPortShared<T> {
    pub fn new(component: T) -> Self {
        Self(component.into())
    }
    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }
    pub fn shared(&self) -> Shared<T> {
        self.0.clone()
    }
    pub fn into_shared(self) -> Shared<T> {
        self.0
    }
}

impl<T: 'static + IndexPort> From<Shared<T>> for IndexPortShared<T> {
    fn from(shared: Shared<T>) -> Self {
        Self(shared)
    }
}
impl<T: IndexPort> Clone for IndexPortShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
pub struct IndexPortRef(Rc<RefCell<dyn IndexPort>>);
impl IndexPortRef {
    pub fn read(&self, index: usize) -> u32 {
        self.0.borrow().read(index)
    }
}
impl<T: 'static + IndexPort> From<Shared<T>> for IndexPortRef {
    fn from(shared: Shared<T>) -> Self {
        Self(shared.into_inner())
    }
}
impl<T: 'static + IndexPort> From<IndexPortShared<T>> for IndexPortRef {
    fn from(shared: IndexPortShared<T>) -> Self {
        shared.into_shared().into()
    }
}
impl Clone for IndexPortRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub trait IndexPortBuilder {
    fn index_connect(&mut self, pin: IndexPortRef, id: usize);
    fn index_alloc(&mut self, id: usize) -> IndexPortRef;
}
