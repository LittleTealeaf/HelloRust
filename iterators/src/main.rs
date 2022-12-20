fn main() {

    {
        let v1 = vec![1,2,3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    {
        let v1 = vec![1,2,3];

        let v1_iter = v1.iter();

        // only impleemnted for certain generic types
        let total: i32 = v1_iter.sum();

        // v1_iter.sum() takes ownership fo the iterator

        println!("Sum is {}", total);
    }
    {
        let v1: Vec<i32> = vec![1,2,3];

        let v2 = v1.iter().map(|x| x + 1);

        for val in v2 {
            println!("Got: {}", val);
        }
    }
    {
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }

        fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }

        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 13,
                style: String::from("sandal")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            }
        ];

        let in_my_size = shoes_in_size(shoes, 10);
        // function takes ownership of the shoes variable

        println!("{:?}", in_my_size);
    }
}

#[cfg(test)]
mod test {
    
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1,2,3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}
