use crate::common::abi::port::index::{IndexPort, IndexPortShared};
use crate::common::abi::utils::Shared;

// use super::utils::ToShared;

pub trait ControlBuilder {
    fn build(self) -> Shared<dyn Control>;
}
#[derive(Default)]
pub struct ControlShared<T: 'static + ?Sized + Control>(Shared<T>);
impl<T: 'static + Control> ControlShared<T> {
    pub fn new(control: T) -> Self {
        Self(Shared::from(control))
    }
    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.0.borrow_mut()
    }
}
// impl<T: ?Sized + Control> ToShared<T> for ControlShared<T> {
//     fn to_shared(self) -> Shared<T> {
//         self.0
//     }
// }
impl<T: 'static + Control> From<T> for ControlShared<T> {
    fn from(control: T) -> Self {
        Self::new(control)
    }
}
impl<T: 'static + Control> From<Shared<T>> for ControlShared<T> {
    fn from(control: Shared<T>) -> Self {
        Self(control)
    }
}
impl<T: 'static + Control + IndexPort> From<IndexPortShared<T>> for ControlShared<T> {
    fn from(shared: IndexPortShared<T>) -> Self {
        Self(shared.to_shared())
    }
}
impl<T: 'static + Control> Clone for ControlShared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
// pub struct ControlRef<T: Control>(SharedRef<T>);
// impl<T: 'static + Control> From<ControlShared<T>> for ControlRef {
//     fn from(shared: ControlShared<T>) -> Self {
//         Self(SharedRef(shared.0 .0))
//     }
// }
// impl<T: 'static + Control + IndexPort> From<IndexPortShared<T>> for ControlRef {
//     fn from(shared: IndexPortShared<T>) -> Self {
//         ControlShared::from(shared).into()
//     }
// }
// impl ControlRef {
//     pub fn rasing_edge(&self) {
//         self.0.borrow_mut().rasing_edge()
//     }
//     pub fn falling_edge(&self) {
//         self.0.borrow_mut().falling_edge()
//     }
//     #[cfg(debug_assertions)]
//     pub fn debug(&self) -> String {
//         self.0.borrow().debug()
//     }
// }

// impl Clone for ControlRef {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }

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
