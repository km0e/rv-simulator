pub trait IndexPort {
    fn read(&self, index: usize) -> u32;
}
pub struct IndexPortShared<T: 'static + IndexPort>(Rc<RefCell<T>>, usize);
impl<T: 'static + IndexPort> Clone for IndexPortShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}
impl<T: IndexPort> IndexPortShared<T> {
    pub fn new(component: T, idx: usize) -> Self {
        Self(Rc::new(RefCell::new(component)), idx)
    }
    pub fn borrow(&self) -> std::cell::Ref<T> {
        (*self.0).borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        (*self.0).borrow_mut()
    }
}
impl<T: 'static + IndexPort> Port for IndexPortShared<T> {
    fn read(&self) -> u32 {
        self.borrow().read(self.1)
    }
}
impl<T: 'static + IndexPort> From<IndexPortShared<T>> for PortRef {
    fn from(shared: IndexPortShared<T>) -> Self {
        PortShared::new(shared).into()
    }
}
pub trait Builder {
    // allocate a new id for the component
    fn alloc(&mut self, id: usize) -> PortRef;
    // connect the component to another component
    fn connect(&mut self, pin: PortRef, id: usize);
    // build the component
    fn build(self) -> Option<ControlRef>;
}
pub trait Port {
    // get data from the component by id
    fn read(&self) -> u32;
}

pub trait Control {
    // update the component
    fn rasing_edge(&mut self) {}
    // update the component
    fn falling_edge(&mut self) {
        self.rasing_edge();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String;
}

#[derive(Default)]
pub struct PortShared<T: 'static + Port>(Rc<RefCell<T>>);
impl<T: 'static + Port> Clone for PortShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Port> PortShared<T> {
    pub fn new(component: T) -> Self {
        Self(Rc::new(RefCell::new(component)))
    }
    pub fn borrow(&self) -> std::cell::Ref<T> {
        (*self.0).borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        (*self.0).borrow_mut()
    }
}
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
pub struct PortRef(Rc<RefCell<dyn Port>>);
impl<T: 'static + Port> From<PortShared<T>> for PortRef {
    fn from(shared: PortShared<T>) -> Self {
        Self(shared.0)
    }
}
impl Clone for PortRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl PortRef {
    pub fn read(&self) -> u32 {
        (*self.0).borrow().read()
    }
}
pub struct ControlRef(Rc<RefCell<dyn Control>>);
impl<T: 'static + Control> From<ControlShared<T>> for ControlRef {
    fn from(shared: ControlShared<T>) -> Self {
        Self(shared.0)
    }
}
impl ControlRef {
    pub fn rasing_edge(&self) {
        (*self.0).borrow_mut().rasing_edge()
    }
    pub fn falling_edge(&self) {
        (*self.0).borrow_mut().falling_edge()
    }
    pub fn debug(&self) -> String {
        (*self.0).borrow().debug()
    }
}
#[derive(Default)]
pub struct ControlShared<T: 'static + Control>(Rc<RefCell<T>>);
impl<T: 'static + Control> ControlShared<T> {
    pub fn new(control: T) -> Self {
        Self(Rc::new(RefCell::new(control)))
    }
    pub fn borrow(&self) -> std::cell::Ref<T> {
        (*self.0).borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        (*self.0).borrow_mut()
    }
}
impl<T: 'static + Control> Clone for ControlShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Clone for ControlRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
mod add;
mod bit;
mod bomb;
mod consts;
mod error;
mod lat;
mod mem;
mod mux;
mod reg;

use core::str;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    ops::Deref,
    pin,
    rc::Rc,
};

pub use add::Add;

pub use mem::Mem;

pub use mux::Mux;

pub use reg::Reg;

use self::lat::Lat;
pub mod build {
    pub use super::add::AddBuilder;
    pub use super::bit::BitBuilder;
    pub use super::consts::ConstsBuilder;
    pub use super::error::Error;
    pub use super::lat::Lat;
    pub use super::mem::Alloc as MemAlloc;
    pub use super::mem::Connect as MemConnect;
    pub use super::mem::MemBuilder;
    pub use super::mux::Alloc as MuxAlloc;
    pub use super::mux::Connect as MuxConnect;
    pub use super::mux::MuxBuilder;
    pub use super::reg::Alloc as RegAlloc;
    pub use super::reg::Connect as RegConnect;
    pub use super::reg::RegBuilder;
    pub use super::Builder;
    pub use super::Port;
    pub use super::PortRef;
    pub use super::PortShared;
}
