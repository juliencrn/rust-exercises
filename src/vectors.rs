pub mod mod_vectors {
    use std::collections::HashMap;

    pub fn demo() {
        // sum -> 403
        let array: Vec<i32> = vec![34, 32, 54, 18, 10, 42, 88, 17, 54, 54, 0];

        println!("Pour le tableau:\n {:?}\n", array);
        println!("-> La moyenne est {}", get_average(&array));
        println!("-> La médiane est {}", get_median(&array));
        println!("-> Le mode est {}", get_mode(&array));
    }

    // output -> 36,636
    fn get_average(array: &Vec<i32>) -> i32 {
        let sum: i32 = array.iter().sum();
        let len = array.iter().len();
        sum / len as i32
    }

    // output -> 34
    fn get_median(array: &Vec<i32>) -> i32 {
        let mut clone = array.clone();
        clone.sort();
        let index = array.iter().len() / 2;
        clone[index]
    }

    // la valeur qui apparaît le plus souvent;
    // output -> 54
    fn get_mode(array: &Vec<i32>) -> i32 {
        // Count each ref we have for each items
        let mut map = HashMap::new();
        for i in array {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }

        // Get the mode
        let mut max = 0;
        let mut mode = 0;
        for (item, count) in map {
            if count > max {
                mode = *item;
                max = count;
            }
        }

        mode
    }
}
