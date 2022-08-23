use core::cell::{RefCell, RefMut};

// NOTICE: UPSafeCell::exclusive_access method
// provides a mutable reference of inner data
// without any "unsafe" statement

// NOTICE: But user should be responsible to 
// guarantee that this is only used in uniprocessor,
// and remember to delete the ref before next exclusive_access
// otherwise a panic will be caused!


pub struct UPSafeCell<T> {
    inner: RefCell<T>,
    // NOTE: RefCell can wrap a non-mut static var
    // so that we can use .borrow/.borrow_mut to get
    // a ref of the var without "unsafe" every time

    // NOTE: But a var wrapped merely by RefCell cant be 
    // set static because it is not safe for sync
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        sef.inner.borrow_mut()
    }
}