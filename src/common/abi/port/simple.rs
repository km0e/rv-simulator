use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::super::utils::Shared;

pub trait Port {
    // get data from the component by id
    fn read(&self) -> u32;
}

#[derive(Default)]
pub struct PortShared<T: 'static + Port>(Shared<T>);
impl<T: 'static + Port> Clone for PortShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Port> PortShared<T> {
    pub fn new(component: T) -> Self {
        Self(component.into())
    }
    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }
    pub fn into_shared(self) -> Shared<T> {
        self.0
    }
}
impl<T: 'static + Port> From<Shared<T>> for PortRef {
    fn from(shared: Shared<T>) -> Self {
        Self(shared.into_inner())
    }
}
pub struct PortRef(Rc<RefCell<dyn Port>>);
impl<T: 'static + Port> From<PortShared<T>> for PortRef {
    fn from(shared: PortShared<T>) -> Self {
        shared.into_shared().into()
    }
}
impl Clone for PortRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl PortRef {
    pub fn read(&self) -> u32 {
        self.0.borrow().read()
    }
}

pub trait PortBuilder {
    type Connect;
    type Alloc;
    fn connect(&mut self, pin: PortRef, id: Self::Connect);
    fn alloc(&mut self, id: Self::Alloc) -> PortRef;
}
