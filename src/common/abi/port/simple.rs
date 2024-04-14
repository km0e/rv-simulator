// use crate::abi::utils::ToShared;
use crate::common::abi::control::{Control, ControlShared};
use crate::common::abi::utils::Shared;
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
        Self(Shared::from(component))
    }
    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.0.borrow_mut()
    }
}
// impl<T: 'static + Port> ToShared<T> for PortShared<T> {
//     fn to_shared(self) -> Shared<T> {
//         self.0
//     }
// }
pub struct PortRef(Shared<dyn Port>);
impl<T: 'static + Port> From<PortShared<T>> for PortRef {
    fn from(shared: PortShared<T>) -> Self {
        Self(shared.to_shared())
    }
}
impl<T: 'static + ?Sized + Port + Control> From<ControlShared<T>> for PortRef {
    fn from(shared: ControlShared<T>) -> Self {
        Self(shared.to_shared().into())
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
    fn connect(&mut self, pin: PortRef, id: usize);
    fn alloc(&mut self, id: usize) -> PortRef;
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MockPort;
    impl Control for MockPort {
        fn debug(&self) -> String {
            unimplemented!()
        }
        fn rasing_edge(&mut self) {}
    }
    impl Port for MockPort {
        fn read(&self) -> u32 {
            unimplemented!()
        }
    }
}
