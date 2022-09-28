fn main() {
//     let s1 = String::from("Hello");

//    let len = calculate_length(&s1);

//    println!("The length of '{}' is {}.", s1, len);

// This code will throw an error

//    let s = String::from("Hello");

//    change(&s);

// End of error code

// Mutable references

//    let mut s = String::from("hello");

//    change(&mut s);

// Note: We can not have two or more mutations occur at the same time

// We can however, use this example to do exactly that. Rust will not compile any data races found in a
// program


// We can NOT have a mutable reference while having a immutable one to the same value
//
// To solve this, we can simply place a println at the end of the immutable references
//
// Then place another println at the end of the mutable reference.

//    let mut s = String::from("Hello");

//    let r1 = &s;
//     let r2 = &s;
//    println!("{}, {} ", r1, r2);

//     let r3 = &mut s;
//    println!("{}", r3);


// Dangling references

// This code will output an error

    let references_to_nothing = dangle();

}

// fn calculate_length(s: &String) -> usize {  // s is a reference to a String
//    s.len()     // Here, s goes out of scope. But it doesn't have ownership of what it refers to,
                // and is not dropped
// }

// fn change(some_string: &mut String) {

//     some_string.push_str(", world");
//

// To fix this, we can simply remove the & or reference
fn dangle() -> String {
    let s = String::from("hello");

    s
}
