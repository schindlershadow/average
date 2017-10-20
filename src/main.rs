fn main() {
    println!("\nGiven a list of integers,\n use a vector and return the mean (average),\n median (when sorted, the value in the middle position),\n and mode (the value that occurs most often; a hash map will be helpful here) of the list\n");

    //list of integers
    let mut integers = vec![1, 3, 2, 4, 5, 6, 7, 8, 9, 10];

    println!("Input Vector");
    print_vec(&integers);

    //sort the vector
    integers.sort();

    println!("Sorted Vector");
    print_vec(&integers);

    let mean = vec_mean(&integers);
    println!("\nmean: {}", mean);

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
