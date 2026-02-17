// Regression test for #142529:
// Function items used as const generic arguments should
// produce a proper error instead of an ICE.

#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

use std::mem::size_of;

fn foo() {}

fn bar<const N: usize>() {}

struct Baz<T, U = [u8; size_of::<T>]>(T, U);
//~^ ERROR: function items cannot be used as const args

fn main() {
    bar::<foo>();
    //~^ ERROR: function items cannot be used as const args
}
