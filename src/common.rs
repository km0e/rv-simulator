pub mod abi;
mod component;
pub use component::build;
pub mod utils;
use core::str;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    ops::Deref,
    pin,
    rc::Rc,
};

use abi::*;
pub trait Composite: Port + Control {}
pub struct CompositeRef(Rc<RefCell<dyn Composite>>);
impl CompositeRef {
    #[cfg(debug_assertions)]
    pub fn debug(&self) -> String {
        (*self.0).borrow().debug()
    }
}
#[derive(Default)]
pub struct CompositeShared<T: 'static + Composite>(Rc<RefCell<T>>);
impl<T: 'static + Composite> Clone for CompositeShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: 'static + Composite> From<CompositeShared<T>> for CompositeRef {
    fn from(shared: CompositeShared<T>) -> Self {
        Self(shared.0)
    }
}
impl<T: 'static + Composite> From<CompositeShared<T>> for PortRef {
    fn from(shared: CompositeShared<T>) -> Self {
        Self(shared.0)
    }
}
impl<T: 'static + Composite> CompositeShared<T> {
    pub fn new(composite: T) -> Self {
        Self(Rc::new(RefCell::new(composite)))
    }
    pub fn borrow(&self) -> std::cell::Ref<T> {
        (*self.0).borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        (*self.0).borrow_mut()
    }
}
