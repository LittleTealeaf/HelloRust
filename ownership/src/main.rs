fn main() {
    //Scope

    {
        // s is not valid
        let _s = "hello";
        // s is valid
    } // scope is over, s is no longer valid

    // Strings
    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    }

    // Variables are freed up once they go out of scope
    

    {
        let x = 5; // bind the value 5 to x
        let y = x; // make a copy of the value of x and bind it to y
    }

    {
        let s1 = String::from("hello");
        let s2 = s1;
        // both will point to the same point in memory (both have the length of 5, and capacity of
        // 5)
        // Rust also no longer considers s1 to be valid after setting to s2.. interesting
    }

    // If we do want to deep copy
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    //Types that implement copy trait (meaning they are not referenced)
    //All integer Types
    //boolean types
    //floats
    //character type
    //tuples if the entire tuple implements the copy trait
    ownership_example();

    // returning values also transfers ownership

    {
        let s1 = String::from("hello");

        let (s1, len) = calculate_length(s1);
    }

}

fn ownership_example() {
    let s = String::from("hello");
    // s comes into scope

    takes_ownership(s);
    // s's value moves into the function
    // and afterwards is no longer valid here

    let x = 5;
    makes_copy(x);
    // x moves into the function
    // but i32 is a copy, so it's ok to use x afterwards
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}


// This is how we return the ownership AND other values
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s,length)
}


// GETTING BACK OWNERSHIP

fn take_back_ownership() {
    let s1 = gives_ownership();
    // value is in s1

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // s2 is moved into the function, which moves and returns
                                       // the value
    
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}



