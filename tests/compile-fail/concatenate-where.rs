#![allow(unused_imports)]

use average::Min;
use average_derive::concatenate;

#[concatenate]
struct Foo
where
    Min: Default,
{
    min: Min,
}

fn main() {}
