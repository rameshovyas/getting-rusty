/*
example showing how lifetime of refernce is fully conatined within the lifetime of variable
This will work
*/
fn main() {
    {
        let x = 5;
        {
            let r = &x;
            assert_eq!(*r, 5);
        }
    }
}
