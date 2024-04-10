pub trait Builder {
    // allocate a new id for the component
    fn alloc(&mut self, id: usize) -> ComponentRef;
    // connect the component to another component
    fn connect(&mut self, pin: ComponentRef, id: usize);
    // build the component
    fn build(self) -> Option<ControlRef>;
}
pub trait Component {
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
}

#[derive(Default)]
pub struct ComponentShared<T: 'static + Component>(Rc<RefCell<T>>);
impl<T: 'static + Component> Clone for ComponentShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Component> ComponentShared<T> {
    pub fn new(component: T) -> Self {
        Self(Rc::new(RefCell::new(component)))
    }
    pub fn borrow(&self) -> std::cell::Ref<T> {
        (*self.0).borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        (*self.0).borrow_mut()
    }
    pub fn alloc(&self) -> usize {
        unimplemented!()
        // (*self.0).borrow_mut().alloc()
    }
    // pub fn alloc_pin(&self) -> Pin {
    //     Pin::new(self.clone().into(), self.borrow_mut().alloc())
    // }
}

pub struct ComponentRef(Rc<RefCell<dyn Component>>);
impl<T: 'static + Component> From<ComponentShared<T>> for ComponentRef {
    fn from(shared: ComponentShared<T>) -> Self {
        Self(shared.0)
    }
}
impl Clone for ComponentRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ComponentRef {
    pub fn read(&self) -> u32 {
        (*self.0).borrow().read()
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
    pub use super::mem::MemBuilder;
    pub use super::mux::MuxBuilder;
    pub use super::reg::RegBuilder;
    pub use super::Builder;
    pub use super::Component;
    pub use super::ComponentRef;
    pub use super::ComponentShared;
}
