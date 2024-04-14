use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct Shared<T: ?Sized>(pub Rc<RefCell<Box<T>>>);

impl<T: ?Sized> Shared<T> {
    pub fn borrow(&self) -> &T {
        &**self.0.borrow()
    }
    pub fn borrow_mut(&self) -> &mut T {
        &mut **self.0.borrow_mut()
    }
}

impl<T> From<T> for Shared<T> {
    fn from(component: T) -> Self {
        Self(Rc::new(RefCell::new(Box::new(component))))
    }
}
impl<T: ?Sized> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
// pub trait ToShared<T: ?Sized> {
//     fn to_shared(self) -> Shared<T>;
// }
// #[derive(Default)]
// pub struct SharedRef<T: ?Sized>(Rc<RefCell<Box<T>>>);

// impl<T: ?Sized> SharedRef<T> {
//     pub fn borrow(&self) -> &T {
//         &**self.0.borrow()
//     }
//     pub fn borrow_mut(&self) -> &mut T {
//         &mut **self.0.borrow_mut()
//     }
// }
// impl<T: ?Sized> Clone for SharedRef<T> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }
// impl<T: ?Sized> From<Shared<T>> for SharedRef<T> {
//     fn from(shared: Shared<T>) -> Self {
//         Self(shared.0.clone())
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_from() {
//         let shared = super::Shared::from(1);
//         let shared_ref: super::SharedRef<i32> = shared.into();
//         assert_eq!(*shared_ref.borrow(), 1);
//     }
// }
