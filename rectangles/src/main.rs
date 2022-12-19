fn main() {
    // First example
    {
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectagnle is {} square pixels",
            area(width1, height1)
        );
    }

    // Second Example
    {
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }

        let rect1 = (30, 50);

        println!("The area of the rectangle is {} square pixels", area(rect1));
    }
    
    // Third example
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("The are of the rectangle is {} square pixels", area(&rect1));


        // printing structs
        
        println!("The struct is {:?}", rect1);

        // format print

        println!("The struct is {:#?}", rect1);


        // alternatively for debugging
        dbg!(&rect1);

        let scale = 2;

        let rect2 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect2);
    }
    

    
}
