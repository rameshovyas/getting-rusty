fn main() {
    //Strings can be created in following ways

    // 1. using to_string() method
    let msg = "I Love Bharat".to_string();
    println!("{}", msg);

    // 2. to_owned() method
    let test = "I Love Bharat".to_owned();
    println!("{}", test);

    // 3. The format!() macro
    let str = format!("{:02}:{:02}:{:02} AM", 5, 34, 25);
    println!("{}", str);

    // 4. concat() method of vector of strings
    let msg = vec!["Ram", "Sita", "Laxman"];
    let s1 = msg.concat();
    println!("{}", s1);

    // 5. join()
    let s2 = msg.join(", ");
    println!("{}", s2);
}
