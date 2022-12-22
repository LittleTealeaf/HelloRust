use std::ops::Deref;
use std::rc::Rc;

fn main() {
    // Using Box<T> to store data on the heap
    {
        let b = Box::new(5);
        println!("b = {}", b);
    }

    //Here, we use Box<T> to make a linked list by instead putting data on the heap rather than the
    //stack
    {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    //Following the pointer to the value

    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        {
            let x = 5;
            let y = MyBox::new(x);

            assert_eq!(5, x);
            assert_eq!(5, *y);
        }

        // Deref Coersion
        {
            fn hello(name: &str) {
                println!("Hello, {name}!");
            }

            {
                let m = MyBox::new(String::from("Rust"));
                hello(&m);
            }
            // If rust didnt' use coersion
            {
                let m = MyBox::new(String::from("Rust"));
                hello(&(*m)[..]);
            }
        }
    }

    // Drop trait
    {
        struct CustomSmartPointer {
            data: String,
        }

        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data `{}`!", self.data);
            }
        }

        {
            let _c = CustomSmartPointer {
                data: String::from("my stuff"),
            };
            let _d = CustomSmartPointer {
                data: String::from("other stuff"),
            };
            println!("CustomSmartPointers created.");

            // Notice how values were dropped in reverse order
        }
        // You can also manually drop values
        {
            let c = CustomSmartPointer {
                data: String::from("my stuff"),
            };
            drop(c);
        }
    }

    // Reference counted smart pointer
    {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};

        {
            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
            println!("count after creating a = {}", Rc::strong_count(&a));
            let b = Cons(3, Rc::clone(&a));
            println!("count after creating b = {}", Rc::strong_count(&a));
            {
                let c = Cons(4, Rc::clone(&a));
                println!("Count after creating c = {}", Rc::strong_count(&a));
            }
            println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
        }
    }

    // Refcells!
    // Rc<T> - Enables multiple owners of the same data; Box<T> and RefCell<T> have single owners
    // Box<T> allows immutable or mutable borrowers checked at compile time; Rc<T> allows only
    // immutable borrowers checked at compile time; RefCell<T> allows immutable or mutable borrows
    // checked at runtime
    //Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value
    //inide the RefCell<T> even when the RefCell<T> itself is immutable
    
}
