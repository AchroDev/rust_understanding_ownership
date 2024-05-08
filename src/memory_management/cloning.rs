/*
   Cloning Avoids Moves
*/

/*
*   One way to avoid moving data is to clone it using the '.clone()' method. For example,
*   we can fix the safety issue in the [Moving_Variables] program with a clone:
*/

fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{} originally, {}", full, first);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

/*
// L1           Heap
Stack
main
first	●   ->  Ferris
first_clone	● -> Ferris

// L2
Stack           Heap
main
first	●   ->  Ferris
first_clone	⦻  // We are cloning and don't need to add anything to the heap
full	●   ->  Ferris
*/

/*
*   Observe that at L1, 'first_clone' did not "shallow" copy the pointer in 'first', but
*   isntead "deep" copied the string data into a new heap allocation. Therefore at L2,
*   while 'first_clone' has been moved and invalidated by 'add_suffix', the original 'first'
*   variable is unchanged. It is safe to continue using 'first'.
*/
