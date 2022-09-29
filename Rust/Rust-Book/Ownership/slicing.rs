fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// String Slice
//fn second_word(s: &String) -> &str {
//    let s = String::from("hello");

//    let hello = &s[0..5];
//    let world = &s[6..11];

    // These two are equal
    // This is how we would start at index zero
    // We can drop the vale before the two periods
//    let slice = &s[0..2];
//    let slice = &s[..2];


    // This slice includes the last byte of String, we can drop the trailing number
//    let len = s.len();

//    let slice = &s[3..len];
//    let slice = &s[3..];

    // We can also drop both values like so.
//    let slice = &s[0..len];
//    let slice = &s[..];
//}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("the first word is: {}", word);
}


// Left off at String Literals are slices
