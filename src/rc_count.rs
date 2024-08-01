#![allow(unused)]
//! substitute with a more advanced example of Rc with cycle issues ?!
use std::{marker::PhantomPinned, mem, rc::Rc};

enum List {
    Nil,
    Cons(i32, Rc<List>),
}

fn main() {
    // use List::{Cons, Nil};

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let _b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let _c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c going out = {}", Rc::strong_count(&a));

    // pinned address keeper with safe interface
    {
        /// 1. Create the value, not yet in an address-sensitive state
        let test_keeper = AddrKeeper::default();
        
        /// 2. Pin the value locally by putting it behind a pinning pointer, thus putting it
        /// it into an address-sensitive state, and subsequently store the initial mem addr
        let mut pinned_test_keeper: Pin<&mut AddrKeeper> = std::pin::pin!(test_keeper);
        
        pinned_test_keeper.as_mut().check_for_move();

        // Trying to access `test_keeper` or pass `pinned_test_keeper` to anything that requires
        // mutable access to a non-pinned version of it will no longer compile
        
        // such common safe code won't compile with the pin interface!
        // let _test_keeper_moved: AddrKeeper = test_keeper; 
        // let _test_keeper_mut_ref: &AddKeeper = &mut test_keeper;
        // let _test_keeper_moved: AddrKeeper = *pinned_test_keeper;
        // let _test_keeper_moved: &mut AddrKeeper = &mut *pinned_test_keeper;

        // Will certainly pass the check with Pin
        pinned_test_keeper.as_mut().check_for_move();
    }

    // self-referential unmovable
    {
        let unmovable: Pin<Box<Unmovable>> = Unmovable::new();

        // The inner pointee `Unmovable` struct will now never be allowed to move.
        // Meanwhile, we are free to move the pointer around.
        let mut still_unmoved = unmovable;
        assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));

        // We cannot mutably dereference a `Pin<Ptr>` unless the pointee is `Unpin` or we use unsafe.
        // Since our type doesn't implement `Unpin`, this will fail to compile.
        // let mut new_unmoved = Unmovable::new();
        // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
    }
}

use std::pin::Pin;

/// the auxillary field of type PhantomPin does not implement std::marker::Unpin trait
/// thus by the machenism of auto trait, the struct is declared to opt out of UnPin
#[derive(Default)]
struct AddrKeeper {
    addr: Option<usize>,
    _pin_phantom: PhantomPinned,
}

impl AddrKeeper {
    /// If addr of the self AddrKeeper has not be checked in the addr field, store the current
    /// address of self instance to the field. Otherwise, assert that the current address of the
    /// self instance is the same as stored value as per its field, causing panic if not true
    pub fn check_for_move(self: Pin<&mut AddrKeeper>) {

        // given self: Pin<&mut AddrKeeper> instance, get the mem addr of self
        // by casting through &AddrKeepr, *const AddrKeepr, and as the usize 
        // of the mem address, as the original mutable reference pointed to
        let curr_addr: usize = ((&*self) as *const AddrKeeper) as usize;        
        
        match self.addr {
            None => {
                unsafe {
                    // SAFETY: it is guaranteed the data pointed to by the obtained mutable reference
                    // through unsafe is not moved 
                    let self_mut_ref: &mut AddrKeeper = self.get_unchecked_mut();
                    self_mut_ref.addr = Some(curr_addr);
                }
            }
            Some(prev_addr) => assert_eq!(prev_addr, curr_addr),
        }
    }
}

fn move_pinned_ref<T>(mut a: T, mut b: T) {
    // This should mean the pointee `a` can never move again.
    let mut pin_ref_mut_a: Pin<&mut T> = unsafe { Pin::new_unchecked(&mut a) };

    // Violating the pin contract on pointee `a` right after
    // The address of `a` changed to `b`'s stack slot, so `a` got moved even though we have previously pinned it!
    mem::swap(&mut a, &mut b); // Potential UB down the road ⚠️

    // let _pinned: Pin<&mut T> = pin_ref_mut_a.as_mut();
}

fn move_pinned_rc<T>(mut x: Rc<T>) {
    // This should mean the pointee can never move again.
    let pin_rc_val: Pin<Rc<T>> = unsafe { Pin::new_unchecked(Rc::clone(&x)) };
    {
        let pin_ref_val: Pin<&T> = pin_rc_val.as_ref();
    }
    drop(pin_rc_val);

    let mut_ref_val = Rc::get_mut(&mut x)
        .expect("there exists other Rc or Weak pointer to the same allocation, failed to obtain mut ref to pointee"); // Potential UB down the road ⚠️
    // if `x` was the only RC to the inner data, we have got a mutable reference to inner data
    // pinned before, which we could use to move it, hence violating the pinning API contract.
 }

use std::ptr::NonNull;
/// This is a self-referential struct because `self.slice` points into `self.data`.
struct Unmovable {
    /// Backing buffer.
    data: [u8; 64],
    /// Points at `self.data` which we know is itself non-null. Raw pointer because we can't do this with a normal reference.
    slice: NonNull<[u8]>,
    // slice_alt: &'a [u8],
    /// Suppress `Unpin` so that this cannot be moved out of a `Pin` once constructed.
    _pin: PhantomPinned,
}

impl Unmovable {
    /// Create a new `Unmovable`.To ensure the data doesn't move we place it on the heap behind a pinning Box.
    /// Note that the data is pinned, but the `Pin<Box<Self>>` which is pinning it can itself still be moved. 
    /// This is important because it means we can return the pinning pointer from the function, which is itself a move!
    fn new() -> Pin<Box<Unmovable>> {
        let res = Unmovable {
            data: [0; 64],
            // We only create the pointer once the data is in place
            // otherwise it will have already moved before we even started.
            slice: NonNull::from(&[]),
            _pin: PhantomPinned,
        };
        // First we put the data in a box, which will be its final resting place
        let mut boxed = Box::new(res);

        // Then we make the slice field point to the proper part of that boxed data.
        // From now on we need to make sure we don't move the boxed data.
        boxed.slice = NonNull::from(&boxed.data);

        // To do that, we pin the data in place by pointing to it with a pinning
        // (`Pin`-wrapped) pointer.
        //
        // `Box::into_pin` makes existing `Box` pin the data in-place without moving it,
        // so we can safely do this now *after* inserting the slice pointer above, but we have
        // to take care that we haven't performed any other semantic moves of `res` in between.
        let pin = Box::into_pin(boxed);

        // Now we can return the pinned (through a pinning Box) data
        pin
    }
}