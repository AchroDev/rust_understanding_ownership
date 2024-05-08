/*
   Variables Cannot Be Used After Being Moved
*/

/*
*   The string program helps illustrate a key safety principle for ownership. Imagine
*   that 'first' were used in 'main' after calling 'add_suffix'. We can simulate such a
*   program and see the undefined behavior that results:
*/

fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{}, originally {}", full, first); // L1 first is now used here after being freed by full
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

/*
// L1
Stack           Heap
main
first  ⦻

full   ●  -->   Ferris Jr.
*/

/*
*   'first' points to deallocated memory after calling 'add_sufix'. Reading 'first' in
*   'println!' would therefore be a violation of memory safety (undefined behavior).
*   Remember: it's not a problem that 'first' points to deallocated memory. It's a problem
*   that we tried to use 'first' after it became invalid.
*
*   Thankfully, Rust will refuse to compile this program, giving the following error:
*/

/*
error[E0382]: borrow of moved value: `first`
 --> test.rs:4:35
  |
2 |     let first = String::from("Ferris");
  |         ----- move occurs because `first` has type `String`, which does not implement the `Copy` trait
3 |     let full = add_suffix(first);
  |                           ----- value moved here
4 |     println!("{full}, originally {first}"); // first is now used here
  |                                   ^^^^^ value borrowed here after move
*/

/*
*   Let's walk through the steps of this error. Rust says that 'first' is moved when we
*   called 'add_suffix(first)' on line 3. The error clarifies that 'first' is moved
*   because it has type 'String', which does not implement 'Copy'. We will discuss 'Copy' soon
*   -- in brief, you would not get this error if you used an i32 instead of a 'String'.
*   Finally, the error says that we use 'first' after being moved (it's "borrowed")
*
*   So if you move a variable, Rust will stop you from using that variable later. More generally,
*   the compiler will enforce this principle

Moved heap data principle: if a variable 'x' moves ownership of heap data to another variable
'y', then 'x' cannot be used after the move


*   Now you should start to see the relationship between ownerships, moves, and safety. Moving
*   ownership of heap data avoids undefined behavior from reading deallocated memory.
*/
