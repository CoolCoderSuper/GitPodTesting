use std::fs;

use simple::*;
use types::*;
use threading::*;
use advfs::*;

pub mod simple;
pub mod types;
pub mod threading;
pub mod advfs;

fn main() {
    basic();
    ownership();
    borrowing();
    type_testing();
    simple_threading();
    advfs_test();
}