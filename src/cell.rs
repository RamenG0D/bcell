use std::{cell::UnsafeCell, mem::transmute};

use crate::token::TokenWith;

pub struct Cell<T, const ID: usize> {
    inner: UnsafeCell<T>,
}

impl<T, const ID: usize> Cell<T, ID> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    /// Reinterpret a `&mut T` into a `&mut Self`. This may be useful if you only need to
    /// temporarily attach a value to a token, for example in a closure.
    pub fn from_mut(m: &mut T) -> &mut Self {
        unsafe { transmute(m) }
    }

    /// Consumes the Cell and returns the inner value. Note that this requires ownership of the Cell,
    /// so it can only be used when you are sure no other references to the Cell exist.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    /// Get a raw pointer to the inner value. This may be useful for FFI or other low-level operations.
    /// Note that this does not provide any safety guarantees, and you must ensure that you do not
    /// violate the borrowing rules of Rust when using this pointer.
    pub fn as_ptr(&self) -> *const T {
        self.inner.get()
    }

    /// Reinterpret a `&self` as a `&T`
    pub unsafe fn get(&self) -> &T {
        unsafe { transmute(self) }
    }

    /// Reinterpret a `&mut self` as a `&mut T`.
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { transmute(self) }
    }

    /// Use a `&Token` to prove no `&mut T` currently exists and recieve a `&T` in return
    pub fn borrow<'lf, 'out, U>(&'lf self, _: &'lf TokenWith<U, ID>) -> &'out T
    where
        'lf: 'out,
    {
        unsafe { self.inner.get().as_ref().unwrap_unchecked() }
    }

    /// Use a `&mut Token` to prove no `&mut T` or `&T` currently exist and recieve a `&mut T` in return.
    pub fn borrow_mut<'lf, 'out, U>(&'lf self, _: &'lf mut TokenWith<U, ID>) -> &'out mut T
	where
		'lf: 'out,
	{
        unsafe { self.inner.get().as_mut().unwrap_unchecked() }
    }
}

/// Very simple debugging function; if you want the inner value, instead use
/// ```println!("{:?}", cell.get(&token))```;
impl<T: std::fmt::Debug + std::any::Any, const ID: usize> std::fmt::Debug for Cell<T, ID> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cell<{}, {}>", std::any::type_name::<T>(), ID)
    }
}
