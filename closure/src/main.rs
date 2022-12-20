use std::thread;

fn main() {
    {
        #[derive(Debug, PartialEq, Copy, Clone)]
        enum ShirtColor {
            Red,
            Blue,
        }

        struct Inventory {
            shirts: Vec<ShirtColor>,
        }

        impl Inventory {
            fn giveaway(&self, user_prefernce: Option<ShirtColor>) -> ShirtColor {
                user_prefernce.unwrap_or_else(|| self.most_stocked())
            }
            fn most_stocked(&self) -> ShirtColor {
                let mut num_red = 0;
                let mut num_blue = 0;

                for color in &self.shirts {
                    match color {
                        ShirtColor::Red => num_red += 1,
                        ShirtColor::Blue => num_blue += 1,
                    }
                }
                if num_red > num_blue {
                    ShirtColor::Red
                } else {
                    ShirtColor::Blue
                }
            }
        }

        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveeaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveeaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }

    {
        fn _add_one_v1(x: u32) -> u32 {
            x + 1
        }
        let _add_one_v2 = |x: u32| -> u32 { x + 1 };
        let _add_one_v4 = |x: u32| x + 1;

        // Closures without parameters are inferred with first user
        let example_closure = |x| x;
        example_closure(String::from("hello world"));
        // example_closure is now a stirng -> string, so it cannot be used with anything else
    }

    {
        // closures can capture values from their environment in three ways

        let list = vec![1,2,3];
        println!("Before defining closure: {:?}", list);

        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("Before calling closure: {:?}", list);
    }

    {
        let mut list = vec![1,2,3];
        println!("Before defining closure: {:?}", list);
        
        let mut borrows_mutably = || list.push(7);

        // cannot borrow it because you can't have a borrow while borrows_mutably has a mutable
        // borrow on the list

        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    // Making closures take ownership of variables
    {
        let list = vec![1,2,3];
        println!("Before defining closure: {:?}", list);

        thread::spawn(move || println!("From thread {:?}", list)).join().unwrap();
    }
    // sorting by key
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        let mut list = [
            Rectangle {width: 10, height: 1},
            Rectangle {width: 3, height: 5},
            Rectangle {width: 7, height: 12},
        ];

        let mut num_sort_operations = 0;

        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("{:#?}, sorted in {num_sort_operations} operations", list);

    }
}
