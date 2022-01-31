pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");

    // You can't directly print an integer like println!(1), you need to give a place holder as i did in this line, most basic format is println!("{}",1)
    println!("Number: {}", 1); 

    //Basic formatting
    println!("{} is from {}", "Yigithan", "Istanbul");

    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}", 
        "Yigithan", "Istanbul", "code"
    );

    //Named Arguments
    println!("{name} likes to play {activity}", name="Yigithan", activity= "Basketball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:0}", 10,10,10);
    println!("{:b}",105);

    //Placeholder for debug trait, (tupal)
    println!("{:?}",(12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {} ", 10 + 10);

    //20.dk da kaldim
}    