/*
    Variables Live in the Stack
*/

/*
*   Here's a program that defines a number 'n' and calls a function 'plus_one' on 'n'.
*   Beneath the program is a new kind of diagram. This diagram visualizes the contents of
*   memory during the program's execution at three marked points.
*/

fn main() {
    let n = 5;
    let y = plus_one(n);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}

/*
*   Variables live in frames. A frame is a mapping from variables to values within a single
*   scope, such as a function. For Example:
*
*   * The frame for 'main' at location L1 holds 'n=5'.
*   * The frame for 'plus_one' at L2 holds 'x = 5'.
*   * The frame for 'main' at location L3 holds 'n = 5; y = 6'.
*
*   Frames are organized into a *stack* of currently-called-functions. For example,
*   at L2 the frame for 'main' sits above the frame for the called function 'plus_one'.
*   After a function returns, Rust deallocates the function's frame. (Dealloccation is also
*   called *freeing* or *dropping*, and we use those terms interchangeably.) This
*   sequence of frames is called a stack because the most recent frame added is always
*   the next frame freed.
*
*   When an expression reads a variable, the variable's value is copied from its slot in the stack frame.
*/

/* STACK FRAME
// L1
Stack
main
a	5

// L2
Stack
main
a	5
b	5

// L3
Stack
main
a	5
b	6
*/
