enum IPAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IPAddrKind,
    address: String,
}

fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    {
        fn route(ip_kind: IPAddrKind) {}

        route(IPAddrKind::V4);
        route(IPAddrKind::V6);
    }

    {
        let home = IpAddr {
            kind: IPAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IPAddrKind::V6,
            address: String::from("::1"),
        };
    }

    {
        enum IpAddr {
            V4(String),
            V6(String)
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));
    } 

    {
        enum IpAddr {
            V4(u8,u8,u8,u8),
            V6(String),
        }

        let home = IpAddr::V4(127,0,0,1);
        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        // Some very fun uses of enum
        

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32,i32,i32),
        }

        impl Message {
            fn call(&self) {

            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    // The option enum
    
    {
        let some_number = Some(5);
        let some_car = Some('e');

        let absent_number: Option<i32> = None;

    }

    // Match control flows
    
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }

    {
        #[derive(Debug)]
        enum UsState {
            Alaska,
            Alabama,
            Connecticut,
            NewYork,
            NewJersey
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState)
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
    }

    // Matching with Option

    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i+1)
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    // Catch all patterns

    {
        let dice_roll = 9;
        match dice_roll {
            3 => {
                println!("Add fancy hat");
            },
            7 => {
                println!("Remove fancy hat");
            },
            _ => {
                println!("Other");
            }
        }


    }
}
