use std::cell::RefCell;

fn foo(x: &RefCell<Box<[u8]>>) {
    let y = x.borrow();
    let z = y.as_ref();

    println!("{:?}", z);
}

fn main() {
    foo(&RefCell::new(Box::new([1, 2, 3_u8])));
}

fn _unrelated_function() {
    some_dependency::useful_api()
}
