fn main() {
    {
        // If you don't initiate values, you need to specify the type with generics!
        let _v: Vec<i32> = Vec::new();
    }
    {
        //You can use vec! to create a new vector with values
        let _v = vec![1, 2, 3];
    }
    {
        //Updating a Vector
        let mut v:Vec<i32> = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        // Reading elements of a vector
        let v = vec![1,2,3,4,5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element"),
        }

        if let Some(value) = v.get(2) {
            println!("The value is {value}");
        } else {
            println!("There is no value");
        }
    }

    {
        // Iterating over values in a vector

        let v = vec![100,32,57];
        for i in &v {
            println!("{i}");
        }
    }
    {
        // Iterating over mutable values in a vector

        let mut v = vec![100,32,57];
        for i in &mut v {
            *i += 50;
        }
        for i in &v {
            println!("{i}");
        }
    }

    {
        // Using enums to store multiple types

        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("test")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    // Dropping a vector drops its elements
}
