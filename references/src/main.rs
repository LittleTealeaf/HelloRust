fn main() {
    {
        fn calculate_length(s: &String) -> usize {
            s.len()
        }

        let s1 = String::from("hello");

        let len = calculate_length(&s1);
        println!("The length of {} is {}", s1, len);
    }

    // We are unable to mutate references. We are not allowed to modify something we have a
    // reference to.

    // unless...

    {
        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }

        let mut s = String::from("hello");

        change(&mut s);
    }
    //restriction of mutable references
    //if you have a mutable reference to a value, you can hav eno other references to that value.
    //The code that attempts to create two mutable references to s will fail
    
    // but we can use scopes

    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1 goes out of scope here

        let r2 = &mut s;
    }

    // similarly, we can't have an immutable reference and try to implement a mutable reference
    

    // Dangling Pointers - a pointer that references a location in meory that may have been given
    // to something else
    
    // if you have dangling pointers, you should just return the value itself
}
