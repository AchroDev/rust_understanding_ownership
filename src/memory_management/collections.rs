/*
   Collections Use Boxes
*/

/*
*   Boxes are used by Rust data structures like 'Vec', 'String', and 'HashMap' to hold
*   a variable number of elements. For example, here's a program that creates, moves, and
*   mutates a string:
*/

fn main() {
    let first = String::from("Ferris"); // L1
    let full = add_suffix(first); // L4
    println!("{}", full);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr."); // L2 before the string, then L3 at the " Jr." string
    name
}

/*
// L1
Stack           Heap
main
first  ●  -->   Ferris

// L2
Stack           Heap
main
first  ●        //First is already in the heap

add_suffix
name   ●   -->  Ferris

// L3
Stack           Heap
main
first  ⦻

add_suffix
name   ●   -->  Ferris Jr.

// L4
Stack           Heap
main
first  ⦻

full   ●   -->  Ferris Jr.
*/

/*
*   This program is more involved, so make sure you follow each step:
*   1. At L1, the string "Ferris" has been allocated on the heap. It is owned by 'first'
*   2. At L2, the function 'add_suffix(first)' has been called. This moves ownership of
*   the string from 'first' to 'name'. The string data is not being copied, but the pointer
*   data is copied.
*   3. At L3, the function 'name.push_str(" Jr.")' resizes the string's heap allocation. This
*   does three things. First, it creates a new larger allocation. Second, it writes "Ferris Jr."
*   into the new allocation, Third. it frees the original heap memory. 'first' now points
*   to deallocated memory.
*   4. At L4 the frame for 'add_suffix' is gone. This function returned 'name', transferring
*   ownership of the string to 'full'.
*/
