// fn main() {                     // s is not valid

    // let s = String::from("hello");            // s is valid from now on

    // println!("{s}");
// }                               // this scope of s is no longer valid


// fn main() {
    // let mut s = String::from("Hello");

    // s.push_str(", world");

    // println!("{}", s);
// }


fn main() {
    // let x = 5;
    // let y = x;

    // let s1 = String::from("hello");
    // let _s2 = s1.clone();

    // println!("x = {}, y = {}", x, y);

    // let s = String::from("hello");

    // takes_ownership(s);

    // let x = 5;

//    makes_copy(x);

//    let s1 = gives_ownership();
//    let s3 = take_and_give_back(s2);

    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

}

//    fn takes_ownership(some_string: String) {
//        println!("{}", some_string);
//    }

//    fn makes_copy (some_integer: i32) {
        // println!("{}", some_integer);
    // }


// Return Values

// fn gives_ownership() -> String {

//    let some_string = String::from("yours");

//    some_string
// }

// fn take_and_give_back(a_string: String) -> String {

//    a_string
// }

fn calculate_length(s: String) -> (String, usize) {

    let length = s.len();

    (s, length)
}
