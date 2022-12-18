fn main() {
    println!("Hello, world!");

    another_function();
    with_parameters(10);
    print_labeled_measurements(5, 'h');
    statement_expression();

    let x = five();
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn with_parameters(x: i32) {
    println!("The value of x is {x}");
}

/*
 * Statements: Instructions that perform an action and do not return a value
 * Expressions: evaluate a resultant value.
 */

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn statement_expression() {
    let y = 0;
    // can't assign statemnts, like
    // let x = (ley y = 0);

    let y = {
        let x = 3;
        x + 1 // IMPORTANT: DOES NOT HAVE SEMICOLON AT END
    };
    println!("The value of y is {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
