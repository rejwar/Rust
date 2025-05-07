struct UnsafeData<T> {
    _marker: std::marker::PhantomData<T>,
}

unsafe impl<T: 'static> Drop for UnsafeData<T> {
    #[may_dangle]
    fn drop(&mut self) {
        println!("Dropping safely without touching dangling memory!");
    }
}
