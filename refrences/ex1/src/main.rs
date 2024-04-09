/*
Try to create reference to a local variable
This will show error : "borrowed value does not live long enough"
*/
fn main() {
    let r;
    {
        let x = 5;
        r = &x; //created reference to x
    }

    assert_eq!(*r, 5);
}

