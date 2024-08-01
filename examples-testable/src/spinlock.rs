#![allow(dead_code, unused)]
use std::{cell::{Cell, UnsafeCell}, ops::{Deref, DerefMut}, path::PathBuf, rc::Rc, sync::{atomic::{AtomicBool, Ordering}, Mutex}, thread::{self, JoinHandle}, time::Duration};

struct WithLockSpinLock<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool,
}

impl<T> WithLockSpinLock<T> {
    fn new(val: T) -> Self {
        Self {
            data: UnsafeCell::new(val),
            locked: AtomicBool::new(false),
        }
    }

    fn with_lock_bogus<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.locked.load(Ordering::Relaxed) != false {
            // spin by doing nothing
        };
        // demonstrate the lack of atomicity of the process of checked the Boolean flag value
        // of lock status and setting the value to denote the locked status by deliberately
        // yielding the current thread in the gap to enhance the possibility that multiple 
        // threads find the lock to be unlocked at the same time at this point s.t. they all 
        // take the lock to proceed without synchronisation with other threads doing the same, 
        // resulting in the averse effect of some updates being overwritten and lost
        thread::yield_now();
        self.locked.store(true, Ordering::Relaxed);
        let ret = f(unsafe {&mut *self.data.get()});
        self.locked.store(false, Ordering::Relaxed);
        ret
    }

    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {

        }
        let ret = f(unsafe {&mut *self.data.get()});
        self.locked.store(false, Ordering::Release);
        ret
    }
}

// SAFETY
unsafe impl<T> Sync for WithLockSpinLock<T> where T: Send {}

struct SpinLockNoData {
    locked: AtomicBool,
}

impl SpinLockNoData {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false)
        }
    }

    pub fn lock(&self) -> () {
        // the implementation would go beyond the while loop and return from the method only if the `swap` call 
        // to store `true` into the atomic bool field returns `false`, implying the acquire of a lock of the 
        // prviously unlocked state
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) -> () {
        self.locked.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use thread::scope;

    use super::*;

    #[test]    
    fn bogus_with_lock_data_race() {
        let boxed_test_data: Box<WithLockSpinLock<u32>> = Box::new(WithLockSpinLock::new(0));
        let ref_mutex_test_data: &'static WithLockSpinLock<u32> = Box::leak(boxed_test_data);

        let handles_vec = (0..10).map(|_| {
            thread::spawn(|| {
                for _ in 0..100 {
                    ref_mutex_test_data.with_lock_bogus(|test_data_counter_mut_ref| {
                        *test_data_counter_mut_ref += 1;
                    });
                }
            })
        }).collect::<Vec<_>>();
        for handle in handles_vec {
            let _ = handle.join();
        }
        
        // data race causing total incremented count not adding up to expectation!
        assert_ne!(ref_mutex_test_data.with_lock_bogus(|test_data_counter_mut_ref| *test_data_counter_mut_ref), 1000)
    }
    
    #[test] 
    fn synchronised_with_lock() {
        let boxed_test_data: Box<WithLockSpinLock<u32>> = Box::new(WithLockSpinLock::new(0));
        let ref_mutex_test_data: &'static WithLockSpinLock<u32> = Box::leak(boxed_test_data);

        thread::scope(|scope_ref| {
            for _ in 0..10 {
                scope_ref.spawn(|| {
                    for _ in 0..100 {
                        ref_mutex_test_data.with_lock(|test_data_counter_mut_ref| {
                            *test_data_counter_mut_ref += 1;
                        });
                    }
                });
            }
            // the thread::scope call implicitly joins in-scope spawned threads
            // if not otherwise done manually, as part of the design of the API
        });

        assert_eq!(ref_mutex_test_data.with_lock_bogus(|test_data_counter_mut_ref| *test_data_counter_mut_ref), 1000)
    }

    #[test]
    fn test_spinlock_nodata() {
        /// Given this version of spin lock implementation only provides the mutex lock
        /// synchronasation functionality, the test demonstration is best done by having
        /// multiple thread mutating a mutable static with the synchonisation provisioned
        /// by the spin lock such that it can seen that data race is eliminated
        static mut TEST_COUNTER_MULTI_THREAD: usize = 0;
        let test_spinlock_nodata = SpinLockNoData::new();
        thread::scope(|scope_ref| {
            for _ in 0..10 {
                scope_ref.spawn(|| {
                    for _ in 0..100 {
                        test_spinlock_nodata.lock();
                        unsafe { TEST_COUNTER_MULTI_THREAD += 1; }
                        test_spinlock_nodata.unlock(); 
                    }           
                });
            }
        });
        assert_eq!(unsafe { TEST_COUNTER_MULTI_THREAD }, 1000)
    }
 
}
