/*
    Boxes Live in the Heap
*/

/*
*   However, copying data can take up a lot of memory. For example, here's a slightly
*   different program. This program copies an array with 1 million elements:
*/

fn main() {
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
*/
