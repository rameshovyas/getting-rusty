/* Program demonstrating formating and print strings */
fn main() {
    // Using println
    println!("Hello");
    println!("Jodhpur");

    // using print 
    print!("Hello");
    println!("Jodhpur");

    // arguments in print
    println!("{} is a city in India", "Jodhpur");

    // multiple arguments in print
    println!("{} is a city in {}", "Jodhpur", "India");

    //positional parameters
    println!("{1} is a city in {0} state of {2}", "Rajasthan", "Jodhpur", "India");

    // minggled behavior of arguments
    println!("{1} {} {0} {}", 10,2);

    /* Compilation error if not used all arguments 
    println!("{1} is a city in state of {2}", "Rajasthan", "Jodhpur", "India");
    */

    // Named placeholders    

    println!("My full name is {first_name} {last_name}", first_name="Ramesh", last_name="Vyas");

    
    
}
