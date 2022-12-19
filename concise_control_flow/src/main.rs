fn main() {
    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {}", max),
            _ => (),
        }
    }
    // can be instead set as
    {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        }
    }
    // We can also do else!

    {
        #[derive(Debug)]
        enum UsState {
            Alaska,
            Alabama,
            Connecticut,
            NewYork,
            NewJersey,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        let mut count = 0;
        let coin = Coin::Penny;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
        
    }
}
