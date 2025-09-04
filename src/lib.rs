pub mod cell;
pub mod token;

pub mod prelude {
    pub use crate::cell::Cell;
    pub use crate::token::{Token, TokenWith};
    pub use crate::{cell, token};
}

pub use iota::iota;

#[macro_export]
/// Create a new `Cell` and associated `Token` with a unique ID.
/// The ID is generated using the `iota` crate, so each invocation of this macro
/// will produce a different ID.
/// This allows you to create multiple cells in the same scope without
/// them interfering with each other.
///
/// Input is a value of any type `T`.
///
/// Output is a tuple of `(Cell<T, ID>, Token<ID>)`.
///
/// # Example
/// ```rust
/// use ccell::cell;
///
/// let (c1, mut t1) = cell!(0);
/// let (c2, mut t2) = cell!(10);
///
/// assert_eq!(*c1.borrow(&t1), 0);
/// assert_eq!(*c2.borrow(&t2), 10);
///
/// *c1.borrow_mut(&mut t1) += 1;
/// *c2.borrow_mut(&mut t2) += 1;
///
/// assert_eq!(*c1.borrow(&t1), 1);
/// assert_eq!(*c2.borrow(&t2), 11);
/// ```
///
/// # Note
///
/// Instances of Cell and Token created with this macro are unique and due to the
/// generated ID creating more cell / tokens may change the id of existing ones
/// however this should not be an issue as the id's are all still unique.
/// If you need a stable ID, use the `new_cell!` and `new_token!` macros instead.
macro_rules! cell {
    ($value:expr) => {{
		cell!(c, t, $value, I);
		(c, t)
	}};

	// allow accessing the created constant ID
	// via hoisting the scope of creation
	($cell:ident, $token:ident, $value:expr, $id:ident) => {
		const $id: usize = $crate::iota!();
		let $cell = $crate::new_cell!($value, $id);
		let $token = $crate::new_token!($id);
	};
}

#[macro_export]
/// Create a new `Cell` with the specified ID.
/// This allows you to create multiple cells in the same scope without
/// them interfering with each other, as long as you ensure the IDs are unique.
///
/// Input is a value of any type `T` and a constant `ID` of type `usize`.
///
///	Output is a `Cell<T, ID>`.
///
/// # Example
/// ```rust
/// use ccell::{new_cell, new_token};
///
/// let c1 = new_cell!("hello", 0);
/// let c2 = new_cell!("world", 1);
///
/// assert_eq!(*c1.borrow(&new_token!(0)), "hello");
/// assert_eq!(*c2.borrow(&new_token!(1)), "world");
/// ```
///
/// # Note
///
/// Instances of Cell created with this macro are not associated with a Token.
/// You must create a Token with the same ID using the `new_token!` macro to
/// access the Cell.
///
/// # NOTICE
///
/// You should use the `cell!` macro instead of this one unless you need
/// to set a specific ID.
macro_rules! new_cell {
    ($value:expr, $id:expr) => {{ $crate::cell::Cell::<_, $id>::new($value) }};
}

#[macro_export]
/// Create a new `Token` with the specified ID.
/// This allows you to create multiple tokens in the same scope without
/// them interfering with each other, as long as you ensure the IDs are unique.
///
/// Input is a constant `ID` of type `usize`.
///
/// Output is a `Token<ID>`.
///
/// # Safety
/// Because tokens represent access (mutable or immutable) to a memory location, creating >1
/// tokens is equivalent to creating >1 mutable references to data.
///
/// # Example
/// ```rust
/// use ccell::new_token;
///
/// let t1 = new_token!(0); // Token<0>
/// let t2 = new_token!(1); // Token<1>
/// ```
///
/// # Note
///
/// Instances of Token created with this macro are not associated with a Cell.
/// You must create a Cell with the same ID using the `new_cell!` macro to
/// access the Cell.
/// If you need a Cell and Token pair, use the `cell!` macro instead.
/// If you need a unique ID, use the `new_cell!` macro instead.
///
/// # NOTICE
///
/// You should use the `cell!` macro instead of this one unless you need
/// to set a specific ID.
macro_rules! new_token {
    ($id:expr) => {{ unsafe { $crate::token::Token::<$id>::new(()) } }};
}
