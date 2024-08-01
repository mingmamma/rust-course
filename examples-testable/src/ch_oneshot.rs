use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::{AtomicBool, Ordering}};

pub struct OneShotChannel<T> {
    item: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    // field meant to model the once-off occurance of a item is sent by
    // one of the threads, s.t. only one send would go through for concurrent
    // sends from multiple threads, as well as eliminating the possibility to
    // manage to do another send sometime after the first send
    used: AtomicBool,
}

// OneShotChannel is designed to be used in multi threaded
// programs s.t. more than one thread can send or receive
// the one-off item in a synchronised manner
unsafe impl<T> Sync for OneShotChannel<T> where T: Send {}

impl<T> OneShotChannel<T> {
    pub const fn new() -> Self {
        Self {
            item: UnsafeCell::new(MaybeUninit::<T>::uninit()),
            ready: AtomicBool::new(false),
            used: AtomicBool::new(false),
        }
    }

    // Safety: this can only be called ONCE!
    pub unsafe fn clumsy_send(&self, item: T) {
        let ref_mut_to_maybe = unsafe { &mut *self.item.get() };
        ref_mut_to_maybe.write(item);
        self.ready.store(true, Ordering::Release);
    }

    pub fn send(&self, item: T) {
        // the Relax ordering is suitable to used on this atomic boolean that is meant to 
        // go through only ONE update in the total modification order of during its lifetime
        if self.used.swap(true, Ordering::Relaxed) {
            // if the return swap call of the previous value is true, it means that a send call has be done before
            // and we are in the position to prevent the another send to proceed
            panic!("cannot send more than once")
        }
        unsafe { (*self.item.get()).write(item); }
    }

    pub fn is_ready_strong(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    // Safety: this can only be used assuming having checked is_ready method returns true
    pub unsafe fn clumsy_recv(&self) -> T {
        let ref_to_maybe = unsafe { &*self.item.get() };
        unsafe { ref_to_maybe.assume_init_read() }
    }

    pub fn recv(&self) -> T {
        todo!()
    }
}

impl<T> Drop for OneShotChannel<T> {
    fn drop(&mut self) {
        todo!()
    }
}