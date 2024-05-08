/*
   A Box's OWner Manages Deallocation
*/

/*
*   Instead, Rust automatically frees a box's heap memory. Here is an almost correct
*   description of Rust's policy for freeing boxes:
*

Box deallocation principle (almost correct): If a variable is bound to a box, when Rust
deallocates the variable's frame, then Rust deallocates the box's heap memory.

*
*   For example, let's trace through a program that allocates and frees a box:
*/

fn main() {
    let a_num = 4; // L1
    make_and_drop(); // L3
}

fn make_and_drop() {
    let a_box = Box::new(5); // L2
}

/*
// L1
Stack
main
a_num	4

// L2
Stack           Heap
main
a_num	4

make_and_drop
a_box	● -->  5

// L3
Stack
main
a_num	4
*/

/*
*   At L1, before calling 'make_and_drop', the state of memory is just the stack frame
*   for 'main'. Then at L2, while calling 'make_and_drop', 'a_box' points to '5' on the
*   heap. Once 'make_and_drop' is finished, Rust deallocates its stack frame. 'make_and_drop'
*   contains the variable 'a_box", so Rust also deallocates the heap data in 'a_box".
*   Therefore the heap is empty at L3.
*
*   The box's heap memory has been successfully managed. But what if we abused this system?
*   Returning to our earlier example, what happens when we bind to variables to a box?
*/

fn example() {
    let a = Box::new([0; 1_000_000]);
    let b = a;
}

/*
*   The boxed array has now been bound to box 'a' and 'b'. By our "almost correct" principle,
*   Rust would try to free the box's heap memory twice on behalf of both variables. That's
*   undefined behavior too!
*
*   To avoid this situation, we finally arrive at ownership. When 'a' is bound to
*   'Box::new([0; 1_000_000])', we say that 'a' *OWNS* the box. The statement
*   'let b = a' *MOVES* ownership of the box from 'a' to 'b'. Given these concepts,
*   Rust's policy for freeing boxes is more accurately described as:

Box deallocation principle (fully correct): If a variable owns a box, when Rust
deallocates the variable's frame, then Rust deallocates the box's heap memory.

*   In the example above, 'b' owns the boxed array. Therefore when the scope ends, Rust
*   deallocates the box only once on behalf of 'b', not 'a'.
*/
