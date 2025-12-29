// Test that the goto chain starting from bb0 is collapsed.
//@ compile-flags: -Cpanic=abort
//@ no-prefer-dynamic

// EMIT_MIR simplify_cfg.main.SimplifyCfg-initial.diff
// EMIT_MIR simplify_cfg.main.SimplifyCfg-post-analysis.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: bb0: {
    // CHECK-NEXT: goto -> bb1;
    // CHECK: bb1: {
    // CHECK: switchInt({{.*}}) -> [0: bb{{[0-9]+}}, otherwise: bb{{[0-9]+}}];
    // CHECK-NOT: unreachable;
    loop {
        if bar() {
            break;
        }
    }
}

#[inline(never)]
fn bar() -> bool {
    true
}
