use std::cell::UnsafeCell;

/// fundamental invariant: this type of once cell is !Sync, trivially accomplished with the
/// struct modelling using UnsafeCell which is !Sync. Furthermore, the struct modelling of 
/// Option<T> (wrapped by UnsafeCell) sufficiently expresses the semantics of once cell:
/// empty ones are None while initialized ones (whose initilization can only be once) are Some
pub struct MyOnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}

impl<T> MyOnceCell<T> {
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    pub fn get(&self) -> Option<&T> {
        let inner_opt_ref: &Option<T> = unsafe {
            // SAFETY
            &*self.inner.get()
        };
        inner_opt_ref.as_ref()
    }

    pub fn set(&self, value: T) -> Result<(), T> {
        let inner_opt_mut_ref: &mut Option<T> = unsafe {
            // SAFETY
            &mut *self.inner.get()
        };
        if inner_opt_mut_ref.is_none() {
            *inner_opt_mut_ref = Some(value);
            Ok(())
        } else {
            Err(value)
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        todo!()
    }

    pub fn into_inner(self) -> Option<T> {
        todo!()
    }

    pub fn take(&mut self) -> Option<T> {
        todo!()
    }

    /// high-level intention of get_or_init related APIs can be
    /// expressed by getting the value from the once_cell, and 
    /// depending on the what's obtained, set the once_cell with 
    /// the computated value of given closure, if neccessary
    pub fn get_or_init(&self, _f: impl FnOnce() -> T) -> &T {
        todo!()
    }

    pub fn try_get_or_init<F, E>(&self, f: F) -> Result<&T, E>
    where F: FnOnce() -> Result<T, E> {
        match self.get() {
            Some(existing) => {
                Ok(existing)
            },
            None => {
                match f() {
                    Err(err) => {
                        Err(err)
                    },
                    Ok(f_val) => {
                        // only interested in using the computed value of closure for initialization
                        // check to make sure the executed closured has not done cell initialization as side effect
                        // which can be conviniently accomplished by inspected the return value of the set call
                        assert!(self.set(f_val).is_ok());
                        Ok(self.get().unwrap())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let test_cell: MyOnceCell<i32> = MyOnceCell::new();
        assert!(test_cell.get().is_none());
        
        assert!(test_cell.set(1).is_ok());
        assert_eq!(test_cell.get(), Some(&1i32));

        assert!(test_cell.set(2).is_err());
        assert_eq!(test_cell.get(), Some(&1i32));
    }
}
