/*
    Boxes Live in the Heap
*/

/*
*   However, copying data can take up a lot of memory. For example, here's a slightly
*   different program. This program copies an array with 1 million elements:
*/

fn heap() {
    let a = [0; 1_000_000]; // L1
    let b = a; // L2
}

/* STACK FRAME
// L1
Stack
main
a	0	0	0	0	0	0	0	0	0	0	0	...	0

// L2
Stack
main
a	0	0	0	0	0	0	0	0	0	0	0	...	0
b	0	0	0	0	0	0	0	0	0	0	0	...	0

Observe that copying a into b causes the main frame to contain 2 million elements.
*/

/* CONT. NOTES
To transfer access to data without copying it, Rust uses pointers. A pointer is a
value that describes a location in memory. The value that a pointer points-to is
called its pointee. One common way to make a pointer is to allocate memory in the
heap. The heap is a separate region of memory where data can live indefinitely.
Heap data is not tied to a specific stack frame. Rust provides a construct called
Box for putting data on the heap. For example, we can wrap the million-element array
in Box::new
*/

fn box_example() {
    let a = Box::new([0; 1_000_000]); // L1
    let b = a; // L2
}

/*
// L1
Stack       Heap
main
a	●  -->  0	0	0	0	0	0	0	0	0	0	0	...	0

// L2
Stack       Heap
main
a	●  // A is being cloned, so it is already in the Heap.
b	●  -->  0	0	0	0	0	0	0	0	0	0	0	...	0
*/

/*
*   Observe that now, there is only ever a single array at a time. At L1, the value of
*   'a' is a pointer (represented by a dot with an arrow) to the array inside the heap.
*   The statement 'let b = a' copies the pointer from 'a' into 'b', but the pointed-to
*   data is not copied. Note that 'a' is now grayed out because it has been moved -- we
*   will see what that means in a moment.
*/
