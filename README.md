
# BCell

A *Better*-Cell implementation (than RefCell) using const generics and zero-cost id's to enforce borrow rules at compile time.

## Whats a RefCell or Cell?

if you prefer to read docs heres links to the rust docs for `Cell` and `RefCell`

* [cell](https://doc.rust-lang.org/std/cell/struct.Cell.html)

* [ref-cell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

First I'll break these into 2 catagories a cell and ref-cell because they both perform different tasks

### Cell

This is pretty much just a transparent wrapper type which allows you to have interior mutability, which just means inside a struct or when not technically declared mutable you can still mutate / change the value.

```rust
let value = std::cell::Cell::new(5usize);

value.set(10); // `value` is now 10 even though `value` isn't mut!
```

### RefCell

first thing to know about a `RefCell` is that it allows dynamic borrow checking, so if you improperly access something at *runtime* your program crashes and explains where. So its literally just a way to move the borrow checker rules to runtime, why would anybody want this?

well in the case of interior mutablity, such as for a *linked list*, you may actually know that you can borrow something however the borrow checker cant ensure that state so it wont let you. However you can use a `RefCell` to tell the compiler to check late (at runtime).

```rust
use std::cell::RefCell;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}

fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    let borrow_one = user_1.active.borrow_mut(); // first mutable borrow - okay
    let borrow_two = user_1.active.borrow_mut(); // second mutable borrow - not okay
}
```

* Example from <https://fongyoong.github.io/easy_rust/Chapter_42.html>

## How it works

So this crate isnt actually a `Cell` but rather a `RefCell` for compile time.

More or less the same as [frankencell](https://github.com/spencerwhite/frankencell) but ill re-explain to have everything in one place.

the concept is to have a `Cell<T, {ID}>` where ID is a const usize such that we can have 2 `Cell`s that are different types, same for `Token<{ID}>`s, so we can create unique types and have a single `Token` that allows for access to a single `Cell`.

So if i was to describe it in a pseudo-math notaton (VERY loosly speaking)

```ignore
new(value) where ID = unique_number()
    => ( cell<ID>(value), token<ID> )
```

Basically when we create a cell and token, we create them both with a unique constant to make them a distinct type and ensure they have the same ID / constant so they can be used together.

How does this ensure borrow safety? Well the unique token itself must be used to create the reference and thats where the normal borrow checker rules apply so:

```rust
let cell  = ...;
let token = ...;

_ = cell.borrow(&token);
// and
_ = cell.borrow_mut(&mut token);
```

the usage of `&token` essential means while the value is borrowed `token` is also borrowed so its valid to make another `&token` but trying to borrow mutably means using a `&mut token` and trying to use an `&mut token` while there are still `&token`s in use is invalid due to borrow checker rules, and if we have a `&mut token` and try to get another mut ref we would have to use another `&mut token` which is also invalid as 2 mutable refs for `token` cannot exist at the same time, or even if we try to regular borrow we still use an `&token` which is invalid when we currently have an `&mut token` in use.

So in short the borrow checker does all the work for us, and the type system ensures no weirdness with other tokens.

## how do we generate a unique constant?

well if you've used rust before you may already know how `const` works and maybe even know about `const fn`'s, if we use common sense and think about how to generate a compile time value in multiple places a `const fn` is the best choice.

So a naive implementation might be something like

```rust
const fn unique() -> usize {
    static mut VALUE: usize = 0;

    let value = unsafe { VALUE }; // store current value

    unsafe {
        VALUE += 1; // update value
    }

    value // return value
}

// expected
assert_eq!(unique(), 0);
assert_eq!(unique(), 1);
assert_eq!(unique(), 2);
assert_eq!(unique(), 3);
// etc...
```

***however***, the rust standard states the `const fn`s are always **pure**, which just means that the output cannot change given the same inputs (including none) AND mutable statics are disallowed.

So given the current rust standard we cannot create a `const fn` that gives a unique value every time its called. Lets do some critical thinking what other constructs of rust happen at compile time?

well theres only two things since `build.rs` is actually **build-time**. The only other option is `proc` or `declarative` macros however declaritive macros dont quite fit the bill either for various reasons.

So we've narrowed down another option, `proc-macro`s unlike the `const fn` standards aren't governed so strictly so static's are allowed, as well as various other methods that might let us make a comp time counter, so how might we implement the unique counter from earlier?

```rust
static mut VALUE: usize = 0;

#[proc-macro]
pub fn unique(_: TokenStream) -> TokenStream {
    let value = unsafe { VALUE }; // store current value

    unsafe {
        VALUE += 1; // update value
    }

    quote! { #value }.into() // return value
}
```

* This does have the added downside of not nessesarily being order deterministic so yeah... however that doesnt really matter for this project :P

Yep, its pretty much the same thing but as a `proc-macro`! It mostly works just because of the principal that `proc-macro`s are basically magic, they are programs which insert code and run as a whole program, so they can kinda do whatever they want.

I have no idea if this is defined behaviour or not, but another option which might be far more defined, would be using `env` vars to create a counter state and return a value.

## documentation

Most functions and structures are documented but there might be places where its missing or not complete (missing constraints, etc.)

## Completeness

The implementation when compared the `std::cell::Cell` or `std::cell::RefCell` is still rather incomplete as it doesnt implement nearly all of the same traits so using it feels somewhat basic. However, it is still usable in that its just a wrapper over its generic type so using borrows the inner types full functionality is all accessible just not transparent.

## Example

```rust
use bcell::{cell::Cell, token::Token};

const ID: usize = ...; // this is actually inside the macro and thus hidden
let (cell, token): (Cell<&str, ID>, Token<ID>) = bcell::cell!("hello, world!"); // the recommended way of creating a cell

// same as
const ID: usize = unique!(); // allows usage of the ID constant if you need it for whatever reason via `hoisting`
let cell : Cell<&str, ID> = bcell::new_cell!("hello, world!", ID);
let token: Token<ID> = bcell::new_token!(ID);

// To use the value (borrow)
let value_ref: &str = *cell.borrow(&token); // borrow it self outputs a &T which if T = &str then &T = &&str so *T = &str
// or mutably
let value_mut: &mut &str = cell.borrow_mut(&mut token); // not very useful here since its a borrowed value
// Line bellow cause comp time error: you cannot borrow token as mutable more than once
// let value_mut2: &mut &str = cell.borrow_mut(&mut token);

println!("Value: {}", value_ref); // prints "hello, world!"
```

* Note: anytime you call `cell!` the returned types are `unique` so you cant do

```rust
let (mut a, mut b) = bcell::cell!('a');
let (c, d) = bcell::cell!('a');

// these error messages are just an example to explain what the actual error messages mean
// they're not the actual error message
a = c; // Error: Type of 'a' is Cell<_, 0> while type of 'c' is Cell<_, 1>
b = d; // Error: Type of 'b' is Token<0> while type of 'd' is Token<1>
```

## Credits

this project was ***HEAVILY*** inspired by the following

* [frankencell](https://github.com/spencerwhite/frankencell)

* [ghost-cell](https://github.com/matthieu-m/ghost-cell)

* [qcell](https://github.com/uazu/qcell)

however the project is most heavily inspired by **frankencell** where the id / method of tracking mutability rules using compile time tokens and ensuring each token and corresponding cell types are unique and only work with each other.

## Star the project?

if you dont mind that is
