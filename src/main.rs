fn main() {
    println!("\nGiven a list of integers,\n use a vector and return the mean (average),\n median (when sorted, the value in the middle position),\n and mode (the value that occurs most often; a hash map will be helpful here) of the list\n");

    //list of integers
    let mut integers = vec![1, 3, 2, 4, 5, 6, 7, 8, 9, 10];

    //sort the vector
    integers.sort();

    print_vec(&integers);
}

fn print_vec(vec: &Vec<u32>) {
    for element in vec.iter() {
        println!("{}", element);
    }
}
