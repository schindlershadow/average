fn main() {
    println!("\nGiven a list of integers,\n use a vector and return the mean (average),\n median (when sorted, the value in the middle position),\n and mode (the value that occurs most often; a hash map will be helpful here) of the list\n");

    //list of integers
    let mut integers = vec![1, 3, 2, 4, 5, 6, 7, 8, 9, 10, 4, 4];

    println!("Input Vector");
    print_vec(&integers);

    //sort the vector
    integers.sort();

    println!("Sorted Vector");
    print_vec(&integers);

    let mean = vec_mean(&integers);
    println!("\nmean: {}", mean);

    let median = vec_median(&integers);
    println!("median: {}", median);

    let mode = vec_mode(&integers);
    println!("mode: {}", mode);
}

fn print_vec(vec: &Vec<i32>) {
    for element in vec.iter() {
        print!("{} ", element);
    }
    println!();
}

fn vec_mean(vec: &Vec<i32>) -> f32 {

    let mut total = 0.0;
    for element in vec.iter() {
        let float = *element as f32;
        total = total + float;
    }

    total / vec.len() as f32
}

fn vec_median(vec: &Vec<i32>) -> f32 {
    let middle_index = (vec.len() / 2) as usize;
    //println!("len {}", vec.len());
    //if vector is even take the average of the middle 2 numbers
    if vec.len() % 2 == 0 {
        let lower_option: Option<&i32> = vec.get(middle_index);
        let upper_option: Option<&i32> = vec.get(middle_index + 1);

        //if vailed indexes
        if let Some(lower) = lower_option {
            if let Some(upper) = upper_option {
                return ((lower + upper)  as f32) / 2.0;
            }
        }
    } else {
        //let middle_index = (vec.len() / 2) as i32;
        let middle_option: Option<&i32> = vec.get(middle_index);

        if let Some(middle) = middle_option {
            return *middle as f32;
        }
    }
    0.0
}

fn vec_mode(vec: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for element in vec.iter() {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    for (key, val) in map.iter() {
        if mode == 0 {
            mode = **key;
        } else if map[&mode] < *val {
            mode = **key;
        }
    }
     mode
}
