use std::ops::Index;

// Example iterator implementation c.f.
// https://doc.rust-lang.org/std/iter/index.html#implementing-iterator
// iterator implementation of Fib num generator s.t.
// result are evaluated & returned lazily
struct FibRecurr1 {
    // the states needed for Fib num generator
    memo: [u64; 2],
    pos: usize,
}

impl Iterator for FibRecurr1 {
    // for Iterator: declare a custom type 
    type Item = u64;

    // for Iterator: implement the next method
    // for mutating the interator internal state
    // and give the returned result
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < 2 {
            let next_val = self.memo[self.pos];
            self.pos += 1;
            Some(next_val)
        }
        else {
            let next_val = self.memo[0] + self.memo[1];
            // state updates:
            // Shift the next val into memo and phase out the old val from memo
            // Increment the pos
            self.memo[0] = self.memo[1];
            self.memo[1] = next_val;
            self.pos += 1;
            Some(next_val)
        }
    }
}

struct FibRecurr2 {
    // initialize an array which is fixed-sized
    memo: [u64;2],
    pos: usize,
}

// how to reason with this lifetime annotation?
struct IndexOffsetHelper<'a> {
    data: &'a [u64;2],
    offset: usize,
}

// how to reason with this lifetime annotation?
impl<'a> Index<usize> for IndexOffsetHelper<'a> {
    // specifying the type of the item being
    // returned as by the indexing op
    type Output = u64;

    // wtf is this?
    #[inline(always)]
    // how to reason with this lifetime annotation 'b?
    fn index<'b>(&'b self, index: usize) -> &'b Self::Output {
        // how to reason the necessitiy of using Wrapping?
        use std::num::Wrapping;

        let index = Wrapping(index);
        let offset = Wrapping(self.offset);
        let two = Wrapping(2);

        let real_index = index - offset + two;
        &self.data[real_index.0]
    }
}

// implement the Iterator trait s.t. the FibRecurr struct works as an iterator
impl Iterator for FibRecurr2 {
    // the required associated type 
    type Item = u64;

    // next is the required method of the Iterator trait
    fn next(&mut self) -> Option<Self::Item> {
        // when the pos is 0 or 1, the next value to return are given by
        // the initial condition argument passed into the macro
        if self.pos < 2 {
            // obtain the value at the pos ready to be returned
            let nextVal = self.memo[self.pos];
            // increment the pos field to be used next time
            self.pos = self.pos + 1;
            // return what's intended
            Some(nextVal)
        } else {
            // We'd like to obtain the nextVal referencing the 
            // recursion condition argument passed into the macro
            // hence the following approach to remap elements e.g.
            // a[n-2], a[n-1] to memo[0], memo[1] s.t.
            // we can get the sum of memo[0] and memo[1] by the form
            // of a[n-2] + a[n-1]
            let nextVal = {
                let n = self.pos;
                // the helper struct is created and assigned to a with inde remapping capacity 
                let a = IndexOffsetHelper{data: &self.memo, offset: n};
                a[n-2] + a[n-1]
            };
            // Now have obtained the nextVal, we need to update memo[0] with memo[1] and
            // get memo[1] updated with nextVal
            {   
                use std::mem::swap;

                // this gem does the three-way swap, which swaps
                // momo[0] into temp in the end
                let mut temp = nextVal;
                // [1, 0] SEEM TO BE array literal?
                for i in [1, 0] {
                    swap(&mut temp, &mut self.memo[i]);
                }
            }
            Some(nextVal)
        }
    }
}



mod test {
    use super::*;

    fn naive_fib(n: usize) -> u64 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            naive_fib(n-1) + naive_fib(n-2)
        }
    }

    #[test]
    fn test_fib_reccur() {
        let mut fib_recurr_struct_1 = FibRecurr1{
            memo: [0, 1],
            pos: 0
        };

        for (i, fib_recurr_struct_ele) in fib_recurr_struct_1.take(10).enumerate() {
            assert_eq!(fib_recurr_struct_ele, naive_fib(i));
        }
    }

    #[test]
    fn test_fib_reccur_2() {
        let mut fib_recurr_struct_2 = FibRecurr2{
            memo: [0, 1],
            pos: 0
        };

        for (i, fib_recurr_struct_ele) in fib_recurr_struct_2.take(10).enumerate() {
            assert_eq!(fib_recurr_struct_ele, naive_fib(i));
        }
    }    
}