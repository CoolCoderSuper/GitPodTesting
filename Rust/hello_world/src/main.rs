use simple::*;
use types::*;
use threading::*;

pub mod simple;
pub mod types;
pub mod threading;

fn main() {
    basic();
    ownership();
    borrowing();
    type_testing();
    simple_threading();
}