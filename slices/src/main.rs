fn main() {
    // Slice lets you reference a continuous sequence of elements in a collection rather than the
    // whole collection

    {
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }

            s.len()
        }

        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear();

        // at this point, word is no longer valid, and is now not in sync with the data
    }
    
    // String slices!

    {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        // these now reference a portion of the string.
    }

    {
        let s = String::from("hello");
        let slice = &s[0..2];
        // same as 
        let slice = &s[..2];
    }

    {
        let s = String::from("hello");
        let len = s.len();
        let slice = &s[3..len];
        // same as 
        let slice = &s[3..];
    }

    {
        let s = String::from("hello");
        let len = s.len();

        let slice = &s[0..len];
        // which is the same as 
        let slice = &s[..];
    }

    // lets rewire first_word now!

    {
        // using &str instead of &String lets us use not just Strings but also slices
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item  == b' ' {
                    return &s[0..i];
                }
            }
            
            &s[..]
        }



    }
}
