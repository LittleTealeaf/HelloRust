use std::collections::HashMap;

fn main() {
    let mut vector = vec![1,3,56,3,4,2,6,5,6,6,7,5,3,4,2,3,4,1,5,7,3,8,4,3,5,3,1,2,3,6,2,3];

    println!("The median is {}", get_median(&mut vector));

    println!("The mode is {}", mode(&vector));

}

fn get_median(array: &mut Vec<i32>) -> &i32 {
    array.sort();
    array.get(array.len() / 2).expect("")
}

fn mode(array: &Vec<i32>) -> i32{
    let mut map: HashMap<i32,i32> = HashMap::new();
    for (_,item) in array.iter().enumerate() {
        map.insert(*item,{
            if let Some(count) = map.get(item) {
                count + 1
            } else {
                1
            }
        });
    }
    
    let mut max: i32 = -1;
    let mut count: i32 = -1;

    for (_,(key,value)) in map.iter().enumerate() {
        if count == -1 || count < *value {
            max = *key;
            count = *value;
        }
    }

    max
}
