fn main() {
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is a green day");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using purple as the background color");
        }
    }

    // While let
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    // For

    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    // Others..
    {
        let (_x, _y, _z) = (1, 2, 3);
        // you are not able to mix and match sizes

        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current Location: ({}, {})", x, y);
        }

        let point = (3, 5);
        print_coordinates(&point);
    }

    // Matching Literals
    {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // Matching named variables
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            // this is a new y value, not the y we stated at the beginning
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {y}", x);
    }

    // multiple patterns and ranges
    {
        let x = 5;

        match x {
            1 | 2 => println!("one or two"),
            3..=6 => println!("three through 6"),
            _ => println!("Something else"),
        }
    }

    // using char values
    {
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    // struct deconstruction

    {
        struct Point {
            x: i32,
            y: i32,
        }

        {
            let p = Point { x: 0, y: 7 };

            let Point { x: a, y: b } = p;

            assert_eq!(0, a);
            assert_eq!(7, b);
        }

        {
            let p = Point { x: 0, y: 7 };

            let Point { x, y } = p;

            assert_eq!(0, x);
            assert_eq!(7, y);
        }
    }

    // Matches with structs
    {
        struct Point {
            x: i32,
            y: i32,
        }

        {
            let p = Point { x: 0, y: 7 };

            match p {
                Point { x, y: 0 } => println!("On the x axis at {x}"),
                Point { x: 0, y } => println!("On the y axis at {y}"),
                Point { x, y } => {
                    println!("On neither axis: ({x}, {y})");
                }
            }
        }
    }
    // Deconstructing enums
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The quit variant has no data to destructure")
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}");
            }
        }
    }
    // Destructuring nested
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r} green {g} and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Chance color to hue {h} saturation {s} and value {v}");
            }
            _ => (),
        }
    }

    // Ignoring values
    {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        foo(3, 4);
    }
    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);
    }
    {
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}");
            }
        }
    }

    // Ignoring remaining parts
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }
    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }

    // Extra conditions in matches)
    {
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("the number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }

        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    // @ bindings
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range");
            }
            Message::Hello { id } => println!("Foudn some other id {}", id),
        }
    }
}
