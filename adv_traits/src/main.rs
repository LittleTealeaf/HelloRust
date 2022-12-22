use std::ops::Add;

fn main() {
    {
        pub trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;
        }

        struct Counter {
            count: u32,
        }
        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }
    }
    {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y
                }
            }
        }
    }
  
    // Default generic type with a Rhs side
    {
        trait Add<Rhs=Self> {
            type Output;
            fn add(self, rhs: Rhs) -> Self::Output;

        }
    }

    // Ambiguity!
    {
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        let person = Human;
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);

        <Human as Pilot>::fly(&person);

    }
}
