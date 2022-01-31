pub fn run() {
    let name = "Brad";
    let mut age = 24;
    println!("My name is {}, and I am {}", name, age);
    age = 23;
    println!("My name is {}, and I am {}", name, age);


    // Define constant
    const ID: i32 = 111; //integer 32
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("My name is {} and my age {}", my_name, my_age);
}