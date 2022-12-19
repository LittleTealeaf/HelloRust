
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    // Structs are cool!
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("someoneelse@example.com");
    // either the entire struct can be mutable or the entire struct can be immutable

    // See examples of how to make a struct below


    {
        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count
        };
        // Better way...
    }
    { 
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("test"),
            ..user1 // TWO .. NOT THREE
        };
    }

    // Tuple Structs
    {
        struct Color(i32,i32,i32);
        struct Point(i32,i32,i32);

        let black = Color(0,0,0);
        let origin = Point(0,0,0);

    }

    // unit-like structs
    // Can be used when needed to implement a trait on some type but dont' have any data that you
    // want to store in the type itself
    {
        struct AlwaysEqual;

        let subject = AlwaysEqual;
    }

    // While you can use references in structs, you will need to indicate "lifetimes" for that data
    // to be present for
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}

fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
