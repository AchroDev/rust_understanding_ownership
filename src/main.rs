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

// Example of a safe program
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

// Entry point
fn main() {
    let x = true;
    read(x);

    //We can make this program unsafe by moving the call to 'read' before the definition of x
}

/*
*   Moving the call to 'read' before the definition of 'x' is unsafe because 'read(x)' expects
*   'x' to have a value of type 'bool', but 'x' doesn't have a value yet.
*
*   When a program like this is executed by an interpreter, then reading 'x' before it's defined
*   would raise an exception such as Python's 'NameError' or Javascript's 'ReferenceError'.
*   But exceptions come at a cost. Each time an interpreted program reads a variable, then
*   the interpreter must check whether that variable is defined.
*
*   Rust's goal is to compile programs into efficent binaries that that require as few runtime
*   checks as possible. Therefore Rust does not check at runtime whether a variable is defined before
*   being used. Instead, Rust checks at compile-time. If you try to compile the unsafe
*   program, you will get an error[E0425]: cannot find value 'x' in this scope. Essentially,
*   letting you know that 'x' isn't defined.
*/
