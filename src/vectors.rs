// Vectors are resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    // Add to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();
    
    // Print them all
    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get vector lenght
    println!("vector Length: {}", numbers.len());

    // vectors are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice {:?}", slice);

    // Loop through vector calues.
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}