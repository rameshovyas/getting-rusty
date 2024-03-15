fn main() {
    println!("Hello, world!");

    /*Using escape sequences */
    let speech = "\"Ouch!\", said the well";
    println!("{}", speech);

    /* multiple line string literals */
    println!(
        "There was a cat
    in the room"
    ); // see the new line will be displayed

    /* using "\"" to drop the new line in multi line string lietral */
    println!(
        "There was a cat \
    in the room",
    ); //see now no two line output
}
