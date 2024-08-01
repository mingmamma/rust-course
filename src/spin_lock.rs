#![allow(dead_code, unused)]
use std::{cell::{Cell, UnsafeCell}, mem, ops::{Deref, DerefMut}, path::PathBuf, rc::Rc, sync::{atomic::{AtomicBool, Ordering}, Mutex}, thread::{self, JoinHandle}, time::Duration};



struct MySpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>
}

// the guard implementation pattern s.t the guard type serves as a replacement of the
// inteneded return value of &mut T (exclusive mutable reference to lock protected data)
// which is implemented by giving the guard type the shared reference to the lock (as its field)
// and delegate the exchange from the &MySpinLock<T> to &mut T by DerefMut implementation
// also noting a struct declaration that contains no pub fields without new-like method implementation
// makes it impossible for caller to contruct the a struct instance since this type is meant to be
// obtained as return of lock method of the mutex spin lock
pub struct MyLockGuard<'a, T> {
    my_lock_ref: &'a MySpinLock<T>
}

impl<'a, T> Deref for MyLockGuard<'a, T> {
    type Target = T;

    fn deref<'b>(&'b self) -> &'b Self::Target {
        // SAFETY?!
        unsafe {
            &*self.my_lock_ref.data.get()
        }
    }
}

impl<T> DerefMut for MyLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY?!
        unsafe {
            &mut *self.my_lock_ref.data.get()
        }
    }
}

impl<T> Drop for MyLockGuard<'_, T> {
    fn drop(&mut self) {
        self.my_lock_ref.locked.store(false, Ordering::Release);
    }
}

impl<T> MySpinLock<T> {
    pub const fn new(val: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(val),
        }
    }

    // be reminded the original intention of design of the lock method of the mutex spin lock
    // is to be return an exclusive mutable reference to the lock protected data to the caller,
    // provided supplying an argument that is a shared reference to the lock instance
    // matching the expected mutex semantics of getting exclusive access to the protected data once the lock is acquired
    // pub fn lock(&self) -> &mut T 
    
    pub fn lock(&self) -> MyLockGuard<T> {
        while self.locked.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            std::hint::spin_loop();
        }
        MyLockGuard {
            my_lock_ref: self
        }
    }
}

// SAFETY
unsafe impl<T> Sync for MySpinLock<T> where T: Send {}

use std::sync::atomic::Ordering::{Acquire, Release, Relaxed};
use std::sync::atomic::AtomicU64;

static mut THREAD_SHARED_DATA: u64 = 0;
static IS_READY: AtomicBool = AtomicBool::new(false);


fn main() {
    // {
    //     thread::spawn(|| {
    //         unsafe { THREAD_SHARED_DATA = 123; }
    //         IS_READY.store(true, Release); // Everything from before this store ..
    //     });
    //     while !IS_READY.load(Acquire) { // .. is visible after this loads `true`.
    //         thread::sleep(Duration::from_millis(10));
    //         println!("waiting for the spawned thread to do and signal...");
    //     }
    //     println!("{}", 
    //         unsafe { THREAD_SHARED_DATA }
    //     );
    // }

    // {
    //     let test_my_spinlock = MySpinLock::new(42);

    //     thread::scope(|s| {
    //         s.spawn(|| {
    //             let mut test_data_mtx_guard = test_my_spinlock.lock();
    //             let test_data_mut_ref = test_data_mtx_guard.deref_mut();

    //             // the drop of the mutex guard guarantees that the references (shared and mutable alike)
    //             // obtained from the mutex guard are invalidated as a result by the virtue that lifetime
    //             // relation of these entities which gurantees that the these reference CANNOT outlive the
    //             // mutex guard instance
    //             drop(test_data_mtx_guard);
    //         });
    //     });
        
    //     let test_data_mut_ref_2 = test_my_spinlock.lock();
    // }
    // {   
    //     use std::ops::Index;
    //     use std::ops::IndexMut;
    //     let mut test_vec = vec!["test".to_string(), "msg".to_string()];
    //     let test_ele_from_vec = test_vec.index(1);
    //     println!("{test_ele_from_vec}");
        
    //     mem::replace(test_vec.index_mut(1), "sinful msg".to_string());
    //     println!("{}", test_vec[1]);
    // }
    {   
        use std::sync::atomic::AtomicPtr;
        
        let raw_ptr = &mut 5 as *mut i32;
        let atm_ptr = AtomicPtr::new(raw_ptr);
        let raw_ptr_back = atm_ptr.load(Ordering::Relaxed);
    }
}



