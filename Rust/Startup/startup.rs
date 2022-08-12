extern crate indicatif;

use std::process::Command;
use indicatif::ProgressBar;


fn main() {

let _test =

    Command::new("zypper")
            .args(["up"])
            .output()
            .expect("failed to execute process");

    Command::new("clear")
            .output()
            .expect("failed to execute process");


let _bar = indicatif::ProgressBar::new(1000);
for _ in 0..1000 {

    _bar.inc(1);
}
_bar.finish();

}







/* fn main() {
    let mut input = String::new();
    println!("Would you like to get started today? ");
    match io::stdin().read_line(&mut input) {
       Ok(_) => {
           println!("You entered: {}", input);
        },
        Err(e) => println!("Please enter a valid option: {}", e),
    }
} */
