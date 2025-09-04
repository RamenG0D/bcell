use std::cell::UnsafeCell;

use bcell::{new_token, prelude::*};

pub struct Arena<T, const ID: usize> {
    inner: UnsafeCell<Vec<T>>,
}

type Index<const ID: usize> = TokenWith<usize, ID>;

impl<T, const ID: usize> Arena<T, ID> {
    fn new(_: TokenWith<U, ID>) -> Self {
        Self {
            inner: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn push(&mut self, value: T) -> Index<ID> {
        let inner = self.inner.get_mut();
        let pos = inner.len();

        inner.push(value);

        unsafe { Index::new(pos) }
    }

    pub fn get(&self, index: &Index<ID>) -> &T {
        let inner = unsafe { self.inner.get().as_ref().unwrap() };

        unsafe { inner.get_unchecked(*index.get()) }
    }

    pub fn get_mut(&self, index: &mut Index<ID>) -> &mut T {
        let inner = unsafe { self.inner.get().as_mut().unwrap() };

        unsafe { inner.get_unchecked_mut(*index.get()) }
    }
}

fn main() {
    const I1: usize = 0;
    const I2: usize = 1;

    let t1 = new_token!(I1);
    let t2 = new_token!(I2);

    let mut chars = Arena::<char, I1>::from(t1);
    let mut nums = Arena::<u32, I2>::from(t2);

    let mut a = chars.push('a');
    let b = chars.push('b');
    let c = chars.push('c');

    let _one = nums.push(1);
    let _two = nums.push(2);

    // This compiles:
    println!("{}", chars.get(&a));
    // This doesn't:
    // println!("{}", chars.get(&_one));

    // This compiles:
    *chars.get_mut(&mut a) = 'ä';
    // This doesn't. With this item, individual items can be declared as mutable or immutable:
    // *chars.get_mut(&mut b) = 'ß';

    // A borrowing pattern not possible with a normal Vec<T>:
    let _a = chars.get_mut(&mut a);
    let _b = chars.get(&b);

    // drop(b);
    // drop(a);

    // Under the hood, an `Index` is just a usize. The following:
    // chars.get(&c);
    // nums.get(&c);
    // is equivalent to:
    // chars.get(2);
    // nums.get(2);
    // However, the `Index` model used here can prove, at compile time, that the following should work:
    let x = chars.get(&c);
    // While this won't:
    // nums.get(&c);

    // and will return the reference without having to check at runtime.
}
