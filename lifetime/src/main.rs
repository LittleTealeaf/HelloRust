use std::fmt::Display;

fn main() {
    // Lifetime Annotation Syntax
    // &i32
    // &'a i32 (reference with explicit lifetime)
    // &'a mut i32 (mutable and explicit lifetime)

    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            // We indicate the generic lifetime annotation 'a
            // We also indicate that x and y should have a lifetime of at least 'a
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let result;

        {
            let string1 = String::from("long string is long");
            {
                let string2 = String::from("xyz");
                result = longest(string1.as_str(), string2.as_str());
                // result only exists until the end of this scope, because the lifetime of result
                // is the same lifetime as string2
                println!("The longest string is {}", result);
            }
        }
    }

    // We can also specify lifetime in structs
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a .");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    // All together now!

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Anmnouncement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
