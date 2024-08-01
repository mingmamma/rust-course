#![allow(dead_code, unused)]

use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    size: usize,
    cap: usize,
}