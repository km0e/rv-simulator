use std::cell::RefCell;
use std::rc::Rc;

use super::Shared;

pub trait ControlBuilder {
    fn build(self) -> ControlRef;
}
#[derive(Default)]
pub struct ControlShared<T: 'static + Control>(Shared<T>);
impl<T: 'static + Control> ControlShared<T> {
    pub fn new(control: T) -> Self {
        Self(control.into())
    }
    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.0.borrow_mut()
    }
    pub fn into_shared(self) -> Shared<T> {
        self.0
    }
}
impl<T: 'static + Control> From<T> for ControlShared<T> {
    fn from(control: T) -> Self {
        Self::new(control)
    }
}
impl<T: 'static + Control> Clone for ControlShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
pub struct ControlRef(Rc<RefCell<dyn Control>>);
impl ControlRef {
    pub fn into_inner(self) -> Rc<RefCell<dyn Control>> {
        self.0
    }
}
impl<T: 'static + Control> From<Shared<T>> for ControlRef {
    fn from(control: Shared<T>) -> Self {
        Self(control.into_inner())
    }
}
impl<T: 'static + Control> From<ControlShared<T>> for ControlRef {
    fn from(shared: ControlShared<T>) -> Self {
        shared.into_shared().into()
    }
}
impl<T: 'static + Control> From<T> for ControlRef {
    fn from(control: T) -> Self {
        Self(Rc::new(RefCell::new(control)))
    }
}
impl ControlRef {
    pub fn rasing_edge(&self) {
        self.0.borrow_mut().rasing_edge()
    }
    pub fn falling_edge(&self) {
        self.0.borrow_mut().falling_edge()
    }
    #[cfg(debug_assertions)]
    pub fn debug(&self) -> String {
        self.0.borrow().debug()
    }
}

impl Clone for ControlRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
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
