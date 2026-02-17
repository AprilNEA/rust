// Regression test for #150749:
// `Self` used in a const generic argument position should
// produce a proper error instead of an ICE.

#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

struct Foo;

impl Foo {
    fn bar() {
        let _: [u8; Self];
        //~^ ERROR: `Self` is not allowed in a const generic argument
    }
}

trait CollectArray<A> {
    fn inner_array(self);
}

impl<A, I> CollectArray<A> for I {
    fn inner_array(self) {
        let _: [A; Self];
        //~^ ERROR: `Self` is not allowed in a const generic argument
    }
}

fn main() {}
