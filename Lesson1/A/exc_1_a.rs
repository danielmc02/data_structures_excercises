

fn main() {
    //Initialize an array to pass to the mystery function
    let ar_example: [i32; 5] = [15, 21, 34, 41, 22];
    //Get a memory safe length value of the array to pass
    let ar_length = ar_example.len();
    println!("The first array is {:?}", ar_example);

    mystery_function(ar_example, ar_length);
}

fn mystery_function(mut array: [i32; 5], n: usize) {
    // The parameter 'n' acts as the size/ length of the array passed

    //init the following int values to act as index positions to use on the array
    let mut i_int: usize = 0;
    let mut j_int: usize = 1;
    //init the following value to equal the first element in the array, this will be used as a reference
    let x_array_ref = array[i_int];
    //run the while loop
    while j_int < n {
        array[i_int] = array[j_int];
        j_int += 1;
        i_int += 1;
    }
    array[i_int] = x_array_ref;
    println!("The new array is {:?}", array);
}
