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
