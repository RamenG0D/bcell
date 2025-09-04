use bcell::{new_cell, new_token, prelude::*};

pub struct LinkedList<'lf, T, const ID: usize> {
    value: T,
    l: Option<&'lf Cell<Self, ID>>,
    r: Option<&'lf Cell<Self, ID>>,
}

impl<'lf, T, const ID: usize> LinkedList<'lf, T, ID> {
	fn new(value: T) -> Self {
		Self {
			value,
			l: None,
			r: None,
		}
	}

	pub fn value(&self) -> &T {
		&self.value
	}

	pub fn left(&self) -> Option<&Cell<Self, ID>> {
		self.l
	}

	pub fn right(&self) -> Option<&Cell<Self, ID>> {
		self.r
	}

	pub fn set_left(&mut self, left: Option<&'lf Cell<Self, ID>>) {
		self.l = left;
	}

	pub fn set_right(&mut self, right: Option<&'lf Cell<Self, ID>>) {
		self.r = right;
	}
}

fn main() {
	const I: usize = 0;

	let mut token = new_token!(I);

	let list1 = new_cell!(LinkedList::new(1), I);

	let mut list2 = LinkedList::new(2);
	list2.set_left(Some(&list1));
    let list2 = new_cell!(list2, I);

    // This compiles:
    list1.borrow_mut(&mut token).r = Some(&list2);

    // This doesn't work (even if list1 is mutable):
    // list1.get_mut().l = Some(&list2);

    // Accessing fields is impossible without the &token.
    let mut current = Some(list1.borrow(&token));
    while let Some(list) = current {
        println!("{}", list.value);

        current = list.r
            .map(|list_ref| list_ref.borrow(&token))
    }
}
