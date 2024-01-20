#[macro_export]
// macro_rules! recurrence {
//     (a[n] = $($inits:expr),+,.,$recur:expr) => {
        
//     };
// }

// Example iterator implementation c.f.
// https://doc.rust-lang.org/std/iter/index.html#implementing-iterator
// iterator implementation of Fib num generator s.t.
// result are evaluated & returned lazily
struct FibRecurr {
    // the states needed for Fib num generator
    memo: [u64; 2],
    pos: usize,
}

impl Iterator for FibRecurr {
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

mod test {
    use super::*;

    #[test]
    fn test_fib_reccur() {
        let mut fib_recurr_struct = FibRecurr{
            memo: [0, 1],
            pos: 0
        };

        for e in fib_recurr_struct.take(5) {
            println!("{}", e);
        }

        assert!(true)
    }


    // fn test_recurrance(){
    //     let fib = recurrence!(a[n] = 0, 1,.,a[n-2] + a[n-1]);
    // }
}