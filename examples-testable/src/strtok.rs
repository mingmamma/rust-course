// recall the 3 current lifetime elision rules:
// - assign a separate lifetime to each input parameter with a reference
// - if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters, 
// which are all references in the return value of the function signature
// - if there are multiple input lifetime parameters but one of them is associated with &self or &mut self, that
// lifetime is assigned to all output lifetime parameters
// in this case, no lifetime ellision rule is applicable, hence explicit lifetime annotation required
fn bogus_strtok<'a>(input_str:&'a mut &'a str, delim: char) -> &'a str {
    match input_str.find(delim) {
        Some(delim_index) => {
            let prefix = &input_str[..delim_index];
            let suffix = &input_str[(delim_index + delim.len_utf8())..];
            *input_str = suffix;
            prefix
        },
        None => {
            let prefix = &input_str[..];
            *input_str = "";
            prefix
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    // utility for guranteeing that the passed in &str arguement's lifetime is fixed to be
    // 'static and not changed by compiler inference to a shorter one 
    fn fix_input_str_ref_to_static(_: &'static str) {}

    #[test]
    fn bogus_strtok_lifetime_issue_demo() {
        let mut s: &'static str = "hello world";
        // let mut s: &str = "hello world";
        
        // fix_input_str_to_static(s);
        
        // an exhibit of lifetime issue with subtype & variance. 
        // the following would fail compilation with the error being borrowed value `s` does not live long enough since
        // `s` is the referent part of the `&'static mut &str` value created by the invokation of strtok_wrong_lifetime()
        // with the passed in argument `s`, but it is clear that the lifetime of `s` is scoped with the test function s.t.
        // the attempt of creation of `&'static mut &str` is rejected by the borrow checker

        // the attempted new mutable reference is 'static by the following reasoning:
        // As a start, `s` is explicitly declared to a value of &'static str type. When passed as arguement to the 
        // strtok_wrong_lifetime call, the following analysis is carried out determine the lifetime `?` of the to-be-created
        // mutable reference &'? mut s, illustrated as the following:
        // fucntion signature parameters with generic lifetime: &'a mut &'a      str, ...
        // arguement supplied:                                  &'? mut &'static str, ...
        // Based on the listed conditions, it can be shown that the lifetime in question is unequivocally `'static` by the folliwng
        // reasoning: Firstly, noting that the generic mutable reference type &mut T is invariant over
        // the generic input type parameter T. By definition, that means even though given a existing relation that
        // U is subtype of T, there is no subtype relation between &mut T and &mut U. Hence for the case in hand, despite
        // that the lifetime 'static is subtype of the undetermined lifetime 'a, and by the given relation that the
        // generic immutable reference type &'a T is convairant over 'a, implying that &'static str is subtype of &'a str,
        // &mut &'static str has no subtype relation with &mut &'a str, forcing 'a to take exactly 'static if it was to be
        // substitued to a concreate value. Additionally, by the extra (incorrectly imposed) constraint given by two 'a,
        // the lifetime ? of &? mut &'static string has be to 'static
     
        // let _ = bogus_strtok(&mut s, ' '); /* Won't compile! */
    }

    #[test]
    fn bogus_strtok_lifetime_no_issue_demo() {
        let test_string= String::from("hello world");
        // this function signature of String: pub fn as_str(&self) -> &str is a epitome of lifetime elision application
        // where the original signature would have been: pub fn as_str<'a>(&'a self) -> &'a str, s.t it is clearly
        // understood that the returned &str lifetime is bounded by the input String arguement's lifetime
        let mut test_string_str = test_string.as_str();
     
        let _ = bogus_strtok(&mut test_string_str, ' '); /* Compiles! */
    }    

    #[test]
    fn how_strtok_works() {
        let mut s: &str = "hello world";
        let hello: &str = bogus_strtok(&mut s, ' ');
        // assert_eq!(s, "world");
        assert_eq!(hello, "hello");
    }
}