pub trait HeapSize {
    fn heap_size_of_children(&self) -> usize;
}

impl HeapSize for u8 {
    /// A `u8` does not own any heap memory.
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

impl<'a, T> HeapSize for &'a T
where
    T: ?Sized,
{
    /// A shared reference does not own heap memory.
    fn heap_size_of_children(&self) -> usize {
        0
    }
}
