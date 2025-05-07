#[no_stack_check]
fn DangerousRecursion() {
    DangerousRecursion(); // This can cause a stack overflow!
}
