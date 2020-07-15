// The Slice Type -------------------------------------------------------------

// Another data type that does not have ownership is the slice.
// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

// String Slices ---
// A string slice is a reference to part of a String, and it looks like this:
fn a() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // hello
    let world = &s[6..11]; // world
}

// This is similar to taking a reference to the whole String but with the extra [0..5] bit.
// Rather than a reference to the entire String, it’s a reference to a portion of the String.

// We can create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index
// is the first position in the slice and ending_index is one more than the last position in the slice.
// Internally, the slice data structure stores the starting position and the length of the slice, which corresponds
// to ending_index minus starting_index. So in the case of let world = &s[6..11];,
// world would be a slice that contains a pointer to the 7th byte (counting from 1) of s with a length value of 5.

// With Rust’s .. range syntax, if you want to start at the first index (zero), you can drop the value before the two periods. In other words, these are equal:
fn b() {
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
}

// By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:
fn c () {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
}

// You can also drop both values to take a slice of the entire string. So these are equal:
fn d() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
}

// Here’s a small programming problem: write a function that takes a string and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
// NOTE: The type that signifies “string slice” is written as &str
fn first_word(s: &String) -> &str {
    // Because we need to go through the String element by element and check whether a value is a space,
    // we’ll convert our String to an array of bytes using the as_bytes method:
    let bytes = s.as_bytes();

    // Next, we create an iterator over the array of bytes using the iter method.
    // iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead.
    // The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element.
    // Because the enumerate method returns a tuple, we can use patterns to destructure that tuple, just like everywhere else in Rust.
    // So in the for loop, we specify a pattern that has i for the index in the tuple and &item for the single byte in the tuple.
    // Because we get a reference to the element from .iter().enumerate(), we use & in the pattern.
    for (i, &item) in bytes.iter().enumerate() {
        // Inside the for loop, we search for the byte that represents the space by using the byte literal syntax.
        // If we find a space, we return a slice from 0 to the current position.
        if item == b' ' {
            return &s[0..i];
        }
    }

    // Otherwise, we return a slice of the entire string
    &s[..]
}

// If we try to use that function and after that, modify the variable s we will get an error.
// We can't modify a mutable reference if we have an immutable borrow of the same variable.
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // -- immutable borrow occurs here

    s.clear(); // error! error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // ^^^^^^^^^ mutable borrow occurs here

    println!("the first word is: {}", word); // ---- immutable borrow later used here
}

// Here is another similar example
fn main_same() {
    let mut mutable = String::from("Hello");

    let immutable_reference = &mutable; // -------- immutable borrow occurs here

    mutable.clear(); // here we try to modify the mutable, but we can't because we already have an immutable reference of the same variable
    // the method clear() needs a mutable value, because it will truncate the String
    // ^^^^^^^^^^^^^^^ mutable borrow occurs here

    println!("{}", immutable_reference); // if this wasn't here, the code will be okay.
                                        // But we need to use the immutable_reference again and the "main" reference was already "deleted"
    // ------------------- immutable borrow later used here
}

// "IF WE HAVE AN IMMUTABLE REFERENCE TO SOMETHING, WE CANNOT ALSO TAKE A MUTABLE REFERENCE."

// String Literals Are Slices ---

// Recall that we talked about string literals being stored inside the binary.
// Now that we know about slices, we can properly understand string literals:
fn literal() {
    let s = "Hello, world!";
}
// The type of s here is &str: it’s a slice pointing to that specific point of the binary.
// This is also why string literals are immutable; &str is an immutable reference.

// String Slices as Parameters ---
// Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:
fn first_word_signature(s: &String) -> &str {}

// A more experienced Rustacean would write the next signature instead because it allows us to use the same function on both &String values and &str values.
fn first_word_better_signature(s: &str) -> &str {}

// If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the entire String.
// Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:
fn main_a() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word_better_signature(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word_better_signature(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_better_signature(my_string_literal);
}

// Other Slices ---
// String slices, as you might imagine, are specific to strings. But there’s a more general slice type, too. Consider this array:
fn f() {
    let a = [1, 2, 3, 4, 5];
}

// Just as we might want to refer to a part of a string, we might want to refer to part of an array. We’d do so like this:
fn g() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // [2, 3]
}
// This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the first element and a length.