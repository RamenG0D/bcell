use bcell::cell;

#[test]
fn test_cell() {
    let (c1, mut t1) = cell!(0);
    let (c2, mut t2) = cell!(4);

    assert_eq!(*c1.borrow(&t1), 0);
    assert_eq!(*c2.borrow(&t2), 4);

    *c1.borrow_mut(&mut t1) += 1;
    *c2.borrow_mut(&mut t2) += 1;

    assert_eq!(*c1.borrow(&t1), 1);
    assert_eq!(*c2.borrow(&t2), 5);

    // the following lines should not compile
    // let _ = c1.borrow(&t2); // Error: mismatched types expected reference `&TokenWith<_, 0>` found reference `&TokenWith<(), 1>`
}

#[test]
fn cell_multi_mut_borrow() {
    let (cell, mut token) = cell!("Bread");

    let borrow1 = cell.borrow_mut(&mut token);
    // should not compile, should have an error message like:
    // let borrow2 = cell.borrow_mut(&mut token); // cannot borrow `token` as mutable more than once at a time second mutable borrow occurs here

    assert_eq!(*borrow1, "Bread");
}

#[test]
fn cell_array_value() {
    let (cell, mut token) = cell!([1, 2, 3, 4, 5]);

    assert_eq!(cell.borrow(&token), &[1, 2, 3, 4, 5]);

    let borrow_mut = cell.borrow_mut(&mut token);
    borrow_mut[0] = 9;

    assert_eq!(cell.borrow(&token), &[9, 2, 3, 4, 5]);

    let borrow = cell.borrow(&token);
    let slice = &borrow[2..4];
    let (cell_reffed, t2) = cell!(slice);

    assert_eq!(*cell_reffed.borrow(&t2), &[3, 4]);
}

#[test]
/// this test ensures even when you have multiple tokens, you can still only
/// access the cell with the correct token
/// AND
/// that with structures containing references to tokens or cell you can still
/// access the cell with the correct token
fn complex_usage() {
    use bcell::{cell, cell::Cell, token::Token};

    struct Wrapper<'a, T, const ID: usize> {
        cell: &'a Cell<T, ID>,
        token: &'a mut Token<ID>,
    }

    impl<'a, T, const ID: usize> Wrapper<'a, T, ID> {
        fn new(cell: &'a Cell<T, ID>, token: &'a mut Token<ID>) -> Self {
            Self { cell, token }
        }
    }

    let (c1, t1) = cell!(0);
    let (c2, mut t2) = cell!(9);

    // should not compile, should have an error message like:
    // Error: mismatched types expected reference `&TokenWith<_, {Number_1}>` found reference `&TokenWith<(), {Number_2}>`
    // where {Number_1} and {Number_2} are different numbers (since the generated ID's arent stable they could kinda be any non matching unique numbers)
    // let c3 = Wrapper::new(&c1, &mut t2);

    assert_eq!(*c2.borrow(&t2), 9);

    let (cell_reffed, t3) = cell!(&c1);
    assert_eq!(*cell_reffed.borrow(&t3).borrow(&t1), 0);

    let wrapper = Wrapper::new(&c2, &mut t2);
    assert_eq!(*wrapper.cell.borrow(wrapper.token), 9);
}
