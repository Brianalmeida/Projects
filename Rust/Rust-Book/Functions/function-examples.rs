// fn five() -> i32 {
   // 5
// }


fn main() {
    // print_labeled_measurement(5, 'h');
    // let x = (let y = 6);

    // let y = {
        // let x = 3;
        // x + 1
    // };

    // println!("The value of y is: {y}");

    // let x = five();

    // println!("The value of x is: {x}");


    let x = plus_five(5);

    println!("The value of x is: {x}");
}


fn plus_five(x: i32) -> i32 {
    x + 1
}

// fn print_labeled_measurement(value: i32, unit_label: char) {
    // println!("The measurement is: {value}{unit_label}");
// }
