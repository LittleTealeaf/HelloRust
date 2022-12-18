fn main() {
    // Needs to be mutable in order to be changed
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // Constants are like immutable variables, but they are not allowed to be mutable with the mut
    // variable.
    // Type has to be annotated
    // It's something like the #define in C/C++
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    //shadowing
    
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");
    
    //We can also use the same name for different types
    
    let spaces = "    ";
    let spaces = spaces.len();

}
