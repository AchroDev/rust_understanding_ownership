/*
    What is Ownership?
*/

/*
*   Ownership is a discipline for ensuring the safety of Rust programs. To understand
*   ownership, we first need to understand what makes a Rust program safe (or unsafe).
*/

/*
    Safety is the Absence of Undefined Behavior
*/

// Example:
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);

    //We can make this program unsafe by moving the call to 'read' before the definition of x
}
