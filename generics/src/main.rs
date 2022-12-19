use std::cmp::PartialOrd;

fn main() {
    {
        fn largest_i32(list: &[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        fn largest_char(list: &[char]) -> &char {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 64];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }
    // using generics
    {
        // We need to indicate that it's a orderable
        fn largest<T: PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 64];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
    // In-struct definitions
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        let _integer = Point { x: 5, y: 10 };
        let _float = Point { x: 1.0, y: 4.0 };
        // Here, both have to be the same type. However, we can fix this with multiple generics
    }
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let _both_integer = Point { x: 5, y: 10 };
        let _both_float = Point { x: 1.0, y: 4.0 };
        let _integer_and_float = Point { x: 5, y: 4.0 };
    }

    //Enums too!
    {
        enum Option<T> {
            Some(T),
            None,
        }

        enum Result<T,E> {
            Ok(T),
            Err(E)
        }
    }
    // In method definitions
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
    }
    
    // Mixing up types
    {
        struct Point<X1,Y1> {
            y: Y1,
            x: X1,
        }

        impl<X1,Y1> Point<X1,Y1> {
            fn mixup<X2,Y2>(self, other:Point<X2,Y2>) -> Point<X1,Y2> {
                Point {
                    x: self.x,
                    y: other.y
                }
            }
        }

        let p1 = Point {x: 5, y: 10.4};
        let p2 = Point {x: "hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
