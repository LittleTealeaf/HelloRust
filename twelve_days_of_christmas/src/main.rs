fn main() {
    let presents = [
        "A partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six gees a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "12 drummers drumming"
    ];

    for day in 1..=12 {
        println!("On the {day} day of christmas, my true love gave to me...");
        for day in (1..day).rev() {
            println!("{}",presents[day]);
        }
    }
}
