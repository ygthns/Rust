// Primitive str: Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you nee to modify or own string data

pub fn run() {
    let mut hello =  String::from("Hello ");
    
    // Get length
    println!("Length: {}, and message is {}", hello.len(), hello);

    hello.push('W'); // character only with push.
    hello.push_str("orld!"); //to push the string

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in  hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len()); // Uzunluk 2 oldugu icin gecer ama farkli bir deger versek test faillenirdi.
    


    println!("{}", s);
}