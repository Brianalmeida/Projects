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

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

}

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn makes_copy (some_integer: i32) {
        println!("{}", some_integer);
    }


// Left off at Return Values
