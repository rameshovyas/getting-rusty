fn main() {
    /***** Variable Bindings *******/ 

    // infering types
    let x = 25;
    println!("{}", x);

    // explicit data type
    let x:f32 = 23.45;
    println!("{}", x);

    // unit type
    let x = ();
    println!("{:?}", x);

    /***** Mutability ****/

    let imutable_variable = 5;
    println!("{}", imutable_variable);
    
    //  cannot assign twice to immutable variable
    /* imutable_variable +=2; cannot assign twice to immutable variable
    println!("{}", imutable_variable);
    */

    // mutable variables
    let mut mutable_variable = 5;
    println!("{}", mutable_variable);

    // Changing mutable variables
    mutable_variable +=2;
    println!("{}", mutable_variable);

    /***** Shadowing *****/
    let shadow_variable= 2;
    println!("Before shadowing : {}", shadow_variable);

    let shadow_variable = "Jodhpur";
    println!("After shadowing : {}", shadow_variable);
}
