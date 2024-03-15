fn main() {
    /* using raw strings to reduce doubleslashes */
    //let path = "c:\Program Files\test"; //error
    let path = r"c:\Program Files\test";
    println!("{}", path);

    /* but we cannot use escape sequence with raw strings */
    //let msg = r"\"Hello!\" again "; // error
    //println!("{}", msg);

    /* to use double quote in raw string  */
    println!(
        r###"
 This raw string started with 'r###"'.
 Therefore it does not end until we reach a quote mark ('"')
 followed immediately by three pound signs ('###'):
"###
    );
}
