/**  Expected user input
  * let fib = recurrence![a[n]: u64 = 0, 1 ;...; a[n-2] + a[n-1]];
  * for e in fib.take(10) { println!("{}", e) }
  */
macro_rules! get_init {
    // Since using repetition with metavariable requires that the repeated metavariable
    // is present in BOTH the matcher and the transcriber in the same number, kind, and order
    // of the repetition, that entails just capturing metavariable repetition in the matcher,
    // and trying to repeat that number of times (for purpose of counting) WITHOUT involving
    // the metavariable in the transcriber, is NOT allowed
    
    // Since the captured metavariable repetition in the matcher CANNOT be used in the transcriber
    // to convey the times of repetition, which CANNOT work with the mandate to also involve the 
    // repeated metavariable in the transcriber, thus a recursive approach is more suitable to place
    // the repeated metavariable in the transcriber in a recursive call
    // ($($inits:expr),+) => {
    //     {
    //         let mut count = 0;
    //         $(count = count + 1)+
    //     }
    // };
    () => {
        0
    };
    ($init:expr) => {
        1
    };
    ($init:expr, $($other_inits:expr),+) => {
        1 + get_init!($($other_inits),+)
    }
}

#[macro_export]
macro_rules! recurrence {
    (a[n]:$dtype:ty = $($inits:expr),+;...;$recur:expr ) => {
        // First of all, we'd expect the transpiler expasion returns an iterator
        {   
            // justify this keyword
            const INIT: usize = get_init!($($inits),+);

            struct FibRecurr2 {
                memo: [u64;2],
                pos: usize,
            }
            
            struct IndexOffsetHelper<'a> {
                data: &'a [u64;2],
                offset: usize,
            }

            use std::ops::Index;
            impl<'a> Index<usize> for IndexOffsetHelper<'a> {

                type Output = u64;

                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b Self::Output {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let two = Wrapping(2);

                    let real_index = index - offset + two;
                    &self.data[real_index.0]
                }
            }     
            
            // Now ready to define the iterator with index utility
            impl Iterator for FibRecurr2 {
                type Item = u64;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.pos < 2 {
                        let next_val = self.memo[self.pos];
                        self.pos = self.pos + 1;
                        Some(next_val)
                    } else {
                    let next_val = {
                        let n = self.pos;
                        let a = IndexOffsetHelper{data: &self.memo, offset: n};
                        a[n-2] + a[n-1]
                    };
                    {   
                        use std::mem::swap;
                        let mut temp = next_val;
                        for i in [1, 0] {
                            swap(&mut temp, &mut self.memo[i]);
                        }
                    }
                    Some(next_val)
                    }
                }
            }
            // In the end, we are done and return the iterator ready to be used
            FibRecurr2 {
                memo: [0, 1],
                pos: 0,
            }
        }
    };
}
  
fn main() {
    let fib = recurrence![a[n]: u64 = 0, 1 ;...; a[n-2] + a[n-1]];
    for e in fib.take(10)  { println!("{}", e) }
}