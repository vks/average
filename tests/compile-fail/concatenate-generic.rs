#![allow(unused_imports)]

use average::Min;
use average_derive::concatenate;

#[concatenate]
struct Foo<T> {
    min: Min,
    _phantom: core::marker::PhantomData<T>,
}

fn main() {}
