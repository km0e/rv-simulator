use std::{cell::RefCell, rc::Rc};

#[derive(Default, Debug)]
pub struct Shared<T: 'static>(Rc<RefCell<T>>);

impl<T: 'static> Shared<T> {
    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.0.borrow_mut()
    }
    pub fn into_inner(self) -> Rc<RefCell<T>> {
        self.0
    }
}

impl<T: 'static> From<T> for Shared<T> {
    fn from(component: T) -> Self {
        Self(Rc::new(RefCell::new(component)))
    }
}
impl<T: 'static> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
