mod slice_type;
mod references_and_borrowing;
mod the_stack_and_the_heap;

fn main() {
    // Ownership ----------------------------------------------------------------------------------

    // Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees
    // without needing a garbage collector. Therefore, it’s important to understand how ownership works in Rust.

    // We’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.

    // All programs have to manage the way they use a computer’s memory while running.
    // Some languages have garbage collection that constantly looks for no longer used memory as the program runs;
    // in other languages, the programmer must explicitly allocate and free the memory.

    // Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.

    // Ownership Rules ---

    // - Each value in Rust has a variable that’s called its owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.
}

// Variable Scope ---

// The variable s refers to a string literal, where the value of the string is hardcoded into the text of our program.
// The variable is valid from the point at which it’s declared until the end of the current scope.

fn do_something() {
    // s is not valid here, it’s not yet declared
    let s = "hello"; // s is valid from this point forward
    // do stuff with s
} // this scope is now over, and s is no longer valid

// In other words, there are two important points in time here:
// - When s comes into scope, it is valid.
// - It remains valid until it goes out of scope.

// The String Type ---
// To illustrate the rules of ownership, we need a more complex data type.

// We’ll use String as the example here and concentrate on the parts of String that relate to ownership.
// These aspects also apply to other complex data types, whether they are provided by the standard library or created by you.

// We’ve already seen string literals, where a string value is hardcoded into our program.
// String literals are convenient, but they aren’t suitable for every situation in which we may want to use text.
// One reason is that they’re immutable. Another is that not every string value can be known when we write our code: for example,
// what if we want to take user input and store it?

// For these situations, Rust has a second string type, String. This type is allocated on the heap and as such is
// able to store an amount of text that is unknown to us at compile time.

// You can create a String from a string literal using the from function, like so:
fn do_another_thing() {
    let s = String::from("hello");
}
// The double colon (::) is an operator that allows us to namespace this particular from function under the
// String type rather than using some sort of name like string_from.

// This kind of string can be mutated:
fn do_another_thing_again() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
// So, what’s the difference here? Why can String be mutated but literals cannot?
// The difference is how these two types deal with memory.

// Memory and Allocation ---

// In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
// This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability.
// Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at
// compile time and whose size might change while running the program.

// With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the
// heap, unknown at compile time, to hold the contents. This means:
// - The memory must be requested from the operating system at runtime.
// - We need a way of returning this memory to the operating system when we’re done with our String.

// That first part is done by us: when we call String::from, its implementation requests the memory it needs.
// This is pretty much universal in programming languages.

// However, the second part is different. In languages with a garbage collector (GC), the GC keeps track and cleans up memory that isn’t
// being used anymore, and we don’t need to think about it. Without a GC, it’s our responsibility to identify when memory is no longer
// being used and call code to explicitly return it, just as we did to request it.

// Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early,
// we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.

// Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

fn do_anything() {
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
} // this scope is now over, and s is no longer valid

// There is a natural point at which we can return the memory our String needs to the operating system: when s goes out of scope.
// When a variable goes out of scope, Rust calls a special function for us.
// This function is called drop, and it’s where the author of String can put the code to return the memory.
// Rust calls drop automatically at the closing curly bracket.

// This pattern has a profound impact on the way Rust code is written.
// It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple
// variables use the data we’ve allocated on the heap.

// Ways Variables and Data Interact: Move ---
// Multiple variables can interact with the same data in different ways in Rust. Let’s look at an example using an integer in Listing 4-2.
fn hello() {
    let x = 5;
    let y = x;
}

// We can probably guess what this is doing: “bind the value 5 to x; then make a copy of the value in x and bind it to y.”
// We now have two variables, x and y, and both equal 5. This is indeed what is happening, because integers are simple values with a known,
// fixed size, and these two 5 values are pushed onto the stack.

// Now let’s look at the String version:
fn bye() {
    let s1 = String::from("hello");
    let s2 = s1;
}

// This looks very similar to the previous code, so we might assume that the way it works would be the same: that is,
// the second line would make a copy of the value in s1 and bind it to s2. But this isn’t quite what happens.

// A String is made up of three parts: a pointer to the memory that holds the contents of the string, a length, and a capacity.
// This group of data is stored on the stack. But in the other side we have the memory on the heap that holds the contents.

// The length is how much memory, in bytes, the contents of the String is currently using.
// The capacity is the total amount of memory, in bytes, that the String has received from the operating system.

// When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
// We do not copy the data on the heap that the pointer refers to.

// Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable.
// But in this case both data pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.
// This is known as a double free error and is one of the memory safety bugs we mentioned previously.
// Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

// To ensure memory safety, there’s one more detail to what happens in this situation in Rust.
// Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and, therefore,
// Rust doesn’t need to free anything when s1 goes out of scope.

// Check out what happens when you try to use s1 after s2 is created; it won’t work:
fn error() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
// we get: error[E0382]: borrow of moved value: `s1`

// If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer,
// length, and capacity without copying the data probably sounds like making a shallow copy.
// But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
// In this example, we would say that s1 was moved into s2.

// That solves our problem! With only s2 valid, when it goes out of scope, it alone will free the memory, and we’re done.
//
// In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data.
// Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

// Ways Variables and Data Interact: Clone ---
// If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

// Here’s an example of the clone method in action:
fn cloning() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

// This works just fine and explicitly produces the behavior where the heap data does get copied.
// When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.
// It’s a visual indicator that something different is going on.

// Stack-Only Data: Copy ---
// There’s another wrinkle we haven’t talked about yet. This code using integers works and is valid:
fn lets_go() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
// But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.

// The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
// That means there’s no reason we would want to prevent x from being valid after we create the variable y.

// In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do
// anything different from the usual shallow copying and we can leave it out.

// Rust has a special annotation called the Copy trait that we can place on types like integers that are stored on the stack.
// If a type has the Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the type,
// or any of its parts, has implemented the Drop trait.
// If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.

// So what types are Copy? as a general rule, any group of simple scalar values can be Copy, and nothing that requires allocation or is some form of resource is Copy.
// Here are some of the types that are Copy:

// - All the integer types, such as u32.
// - The Boolean type, bool, with values true and false.
// - All the floating point types, such as f64.
// - The character type, char.
// - Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.


// Ownership and Functions ---
// The semantics for passing a value to a function are similar to those for assigning a value to a variable.
// Passing a variable to a function will move or copy, just as assignment does.

fn main_function() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);     // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                    // x comes into scope

    makes_copy(x);          // x would move into the function,
    // but i32 is Copy, so it’s okay to still use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error. These static checks protect us from mistakes.
// Try adding code to main that uses s and x to see where you can use them and where the ownership rules prevent you from doing so.

// Return Values and Scope ---
// Returning values can also transfer ownership.

fn another_main_function() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return another String
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

// The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
// When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

// Taking ownership and then returning ownership with every function is a bit tedious.
// What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again,
// in addition to any data resulting from the body of the function that we might want to return as well.

// It’s possible to return multiple values using a tuple:
fn main_two() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
// But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for this concept, called references.
