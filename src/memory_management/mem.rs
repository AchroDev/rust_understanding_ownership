/*
    Rust Does Not Permit Manual Memory Management
*/

/*
*   Memory management is the proces of allocating memory and deallocating memory. In
*   other words, it's the process of finding unused memory and later returning that memory
*   when it is no longer used. Stack frames are automatically managed in Rust. When a
*   function is called, Rust allocates a stack frame for the called function. When the call
*   ends, Rust deallocates the stack frame.
*
*   As we saw in [The Heap], heap data is allocated when calling 'Box::new(..)'. But
*   when is heap data deallocated? Imagine that Rust had a 'free()' function that frees
*   a heap allocation. Imagine that Rust let a programmer call 'free' whenever they wanted.
*   This kind of "manual" memory management easily leads to bugs. For example, we could read
*   a pointer to freed memory:
*/

// This program does not compile and is just an example
fn free() {
    let b = Box::new([0; 100]); // L1
    free(b); // L2
    assert!(b[0] == 0); // !L3!
}

/*
// L1
Stack       Heap
main
b	●  -->  0	0	0	0	0	0	0	0	0	0	0	...	0

// L2
Stack
main
b	⦻

// L3 !undefined behavior!: pointer used after its pointee is freed
Stack
main
b	⦻
*/

/*
*   Here we allocate an array on the heap. Then we call 'free(b)', which deallocates
*   the heap memory of 'b'. Therfore the value of 'b' is a pointer to invalid memory,
*   which we represent as the "⦻" icon. No undefined behavior has happened yet! The
*   program is still safe at L2. It's not necessarily a problem to have an invalid pointer.
*
*   The undefined behavior happens when we try to use the pointer by reading 'b[0]'. That
*   would attempt to access invalid memory, which could cause the program to crash. Or
*   worse, it could not crash and return arbitrary data. Therefore this program is *unsafe*.
*
*   Rust does not allow programs to manually deallocate memory, That policy avoids the kinds
*   of undefined behaviors shown above.
*/
