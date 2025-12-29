//@ test-mir-pass: GVN
//@ only-64bit
//@ compile-flags: -Z mir-enable-passes=+Inline

// Regression for <https://github.com/rust-lang/rust/issues/127089>

struct Foo<T>(std::marker::PhantomData<T>);

impl<T> Foo<T> {
    const SENTINEL: *mut T = std::ptr::dangling_mut();

    fn cmp_ptr(a: *mut T) -> bool {
        std::ptr::eq(a, Self::SENTINEL)
    }
}

// EMIT_MIR gvn_ptr_eq_with_constant.main.GVN.diff
pub fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: (inlined Foo::<u8>::cmp_ptr)
    // CHECK: [[ptr:_.*]] = const {0x1 as *mut u8};
    // CHECK: [[a:_.*]] = const {0x1 as *const u8};
    // CHECK: [[b:_.*]] = const {0x1 as *const u8};
    // CHECK: {{_.*}} = const true;
    Foo::<u8>::cmp_ptr(std::ptr::dangling_mut());
}
