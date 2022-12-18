fn main() {
    // Rust throws an error if we do not indicate a value type
    let guess: u32 = "42".parse().expect("Not a Number!");

    //Scalar Types

    //Integers
    let eight_bit: i8 = 0;
    let sixteen_bit: i16 = 0;
    let thirtytwo_bit: i32 = 0;
    let sixtyfour_bit: i64 = 0;
    let onetwentyeight_bit: i128 = 0;
    let arch_bit: isize = 0;
    // For unsigned, use u___
    // signed numbers are stored with two's compliment

    //You can also use underscores to make reading easier
    let really_large_number: i64 = 1_000_000;

    //Floating Point Numbers

    let x = 2.0;

    let y: f32 = 3.0;

    //Stored in IEEE-754 standard

    //Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    //Booleans

    let t = true;
    let f: bool = false;

    //Characters!

    let c = 'z';
    let z: char = 'Z';
    //You can use emojis as chars
    // Chars in single quotes, not double quotes. Double quotes are strings

    // TUPLES

    let tup = (500, 6.4, 1);
    let _tup_declared: (i32, f64, u8) = (500, 6.4, 1);

    //You can unpack like javascript
    let (x, y, z) = tup;
    println!("The value of y is {y}");

    //Accessing tuple elements
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //Arrays
    let a = [1, 2, 3, 4, 5];
    //Fixed length, same type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    //You can initialize array with the same values:
    let a = [3; 5];
    //a will contain 5 elements, each equalling 3
    let first = a[0];
    let second = a[1];
    //Runtime error if you try to access past the length
}
