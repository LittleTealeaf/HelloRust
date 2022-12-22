fn main() {
    {
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);
    }

    {
        let list_of_numbers = vec![1,2,3];

        let _list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

        let _list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect(); // better

    }

    // Closures are repsented by traits, so you can't return closures directly. But, you can return
    // it as a box?
    {
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}
