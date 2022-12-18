fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number divisible by 4");
    } else if number % 3 == 0 {
        println!("number divisible by 3");
    } else if number % 2 == 0 {
        println!("number divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // TERNARY OPERATORS ARE NO MORE, IT'S BUILT IN!

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");

    // Results from a condition must all be the same data type

    // breaks out of loop with a break, like...

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");


    // LOOP LABELS (WAIT, THEY HAVE BETTER LOOP STUFF THAN OTHER LANGUAGES?)
    
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loops

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF");

    // raw looping

    let a = [10,20,30,40,50,60];
    let mut index = 0;

    while index < 6 {
        println!("The value is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is {element}");
    }

    // looping in range
    //
    // RANGES: 1..4  -> range from 1,2,3
    // 1..=4 -> range from 1, 2, 3, 4

    for number in 1..4 {
        println!("{number}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}

fn endless_loop() {
    loop {
        println!("again!");
    }
}
