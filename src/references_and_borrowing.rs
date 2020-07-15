// References and Borrowing ---
// The issue with the tuple code in the previous example is that we have to return the String to the calling function so we can still use the String after
// the call to calculate_length, because the String was moved into calculate_length.

// Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

fn main_three() {
    let s1 = String::from("hello");

    let len = calculate_length_two(&s1); // we use & to send the value as a reference

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_two(s: &String) -> usize { // we use & to receive a value as a reference, s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

// First, notice that all the tuple code in the variable declaration and the function return value is gone.
// Second, note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String.

// These ampersands (&) are references, and they allow you to refer to some value without taking ownership of it.
// Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

// The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
// Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.

// The scope in which the variable s is valid is the same as any function parameter’s scope,
// but we don’t drop what the reference points to when it goes out of scope because we don’t have ownership.
// When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

// We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

// So what happens if we try to modify something we’re borrowing?
fn main_four() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    // some_string.push_str(", world"); <--------- error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

// Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

// Mutable References ---
// We can fix the previous error in the code with just a small tweak:
fn main_five() {
    let mut s = String::from("hello");

    change_two(&mut s);
}

fn change_two(some_string: &mut String) {
    some_string.push_str(", world");
}

// First, we had to change s to be mut. Then we had to create a mutable reference with &mut s and accept a mutable reference with some_string: &mut String.

// But mutable references have one big RESTRICTION: you can have only one mutable reference to a particular piece of data in a particular scope.
// This code will fail:
fn fail() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time

    println!("{}, {}", r1, r2);
}

// This restriction allows for mutation but in a very controlled fashion.
// It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like.

// The benefit of having this restriction is that Rust can prevent data races at compile time.
// A data race is similar to a race condition and happens when these three behaviors occur:

// - Two or more pointers access the same data at the same time.
// - At least one of the pointers is being used to write to the data.
// - There’s no mechanism being used to synchronize access to the data.

// Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime;
// Rust prevents this problem from happening because it won’t even compile code with data races!

// As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
fn asd() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

// A similar rule exists for combining mutable and immutable references.
fn dsa () {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

// Whew! We also cannot have a mutable reference while we have an immutable one.
// Users of an immutable reference don’t expect the values to suddenly change out from under them!
// However, multiple immutable references are okay because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

// Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
// For instance, this code will compile because the last usage of the immutable references occurs before the mutable reference is introduced:
fn ddas() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // The scopes of the immutable references r1 and r2 ends here, where they are last used
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem, These scopes don’t overlap, so this code is allowed.
    println!("{}", r3);
}

// Dangling References ---
// In languages with pointers, it’s easy to erroneously create a dangling pointer, a pointer that references a location in memory that may have been given to someone else,
// by freeing some memory while preserving a pointer to that memory.

// In Rust, by contrast, the compiler guarantees that references will never be dangling references:
// if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

// Let’s try to create a dangling reference, which Rust will prevent with a compile-time error:

fn main_six() {
    let reference_to_nothing = dangle();
}

// dangle returns a reference to a String
fn dangle() -> &String { // error[E0106]: missing lifetime specifier, help: consider giving it a 'static lifetime: `&'static`
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away. DANGER!

// in other words: this function's return type contains a borrowed value, but there is no value for it to be borrowed from.

// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
// But we tried to return a reference to it. That means this reference would be pointing to an invalid String.
// That’s no good! Rust won’t let us do this.

// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
// This works without any problems. Ownership is moved out, and nothing is deallocated.

// The Rules of References ---
// Let’s recap what we’ve discussed about references:

// - At any given time, you can have either one mutable reference or any number of immutable references.
// - References must always be valid.
