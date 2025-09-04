use std::marker::PhantomData;

use crate::cell::Cell;

/// A proof exclusive (mutable) or shared (immutable) access to a memory location.
/// The `ID` generic parameter is used to create multiple distinct cell types that can
/// be used in the same scope.
pub struct TokenWith<T, const ID: usize>(
	pub T,
	PhantomData<()>,
);

// A token that does not carry any data.
// This is a type alias for TokenWith<(), ID>.
pub type Token<const ID: usize> = TokenWith<(), ID>;

impl<T, const ID: usize> TokenWith<T, ID> {
    /// Create a new token
    ///
    /// # Safety
    /// Because tokens represent access (mutable or immutable) to a memory location, creating >1
    /// tokens is equivalent to creating >1 mutable references to data.
    pub const unsafe fn new(value: T) -> Self {
        Self(value, PhantomData)
    }

    /// Reinterpret a `&mut TokenWith<U, ID>` into a `&mut TokenWith<T, ID>`.
    /// This may be useful if you only need to temporarily attach a value to a token, for example in a closure.
    pub fn cell(&self, data: T) -> Cell<T, ID> {
        Cell::new(data)
    }

	/// Get a reference to the inner value.
	pub fn get(&self) -> &T {
		&self.0
	}

	/// Get a mutable reference to the inner value.
	pub fn get_mut(&mut self) -> &mut T {
		&mut self.0
	}

	/// Consume the token and return the inner value.
	pub fn into_inner(self) -> T {
		self.0
	}
}
