use std::{
    cell::UnsafeCell,
    ptr::{self, NonNull},
};

use super::StrongPtr;
use crate::runtime::{self, Object};

// Our pointer must have the same address even if we are moved, so Box it.
// Although loading the WeakPtr may modify the pointer, it is thread safe,
// so we must use an UnsafeCell to get a *mut without self being mutable.

/// A pointer that weakly references an object, allowing to safely check
/// whether it has been deallocated.
pub struct WeakPtr(Box<UnsafeCell<NonNull<Object>>>);

impl WeakPtr {
    /// Constructs a `WeakPtr` to the given object.
    /// Unsafe because the caller must ensure the given object pointer is valid.
    /// If `obj` is `null`, then this function returns `None`.
    pub unsafe fn new(obj: *mut Object) -> Option<Self> {
        let ptr = ptr::null_mut();
        runtime::objc_initWeak(&mut ptr, obj);
        NonNull::new(ptr)
            .map(UnsafeCell::new)
            .map(Box::new)
            .map(WeakPtr)
    }

    /// Loads the object self points to, returning a `StrongPtr`.
    /// If the object has been deallocated, this function will return `None`.
    pub fn load(&self) -> Option<StrongPtr> {
        unsafe {
            let ptr = runtime::objc_loadWeakRetained(self.0.get().as_ptr());
            StrongPtr::new(ptr)
        }
    }
}

impl Drop for WeakPtr {
    fn drop(&mut self) {
        unsafe {
            runtime::objc_destroyWeak(self.0.get().as_ptr());
        }
    }
}

impl Clone for WeakPtr {
    fn clone(&self) -> Self {
        let ptr = ptr::null_mut();
        unsafe {
            runtime::objc_copyWeak(&mut ptr, self.0.get().as_ptr());
            WeakPtr::new(ptr).expect("runtime::objc_copyWeak unexpectedly failed")
        }
    }
}
