use std::convert::From;

fn main() {
    let my_str = "hello";
    println!("{:?}",my_str);

    let my_string = String::from(my_str);
    println!("{:?}",my_string);

}
