mod log;
mod log_macro;
mod test;


fn main() {
    println!("Hello, world!");

    // std::string::String::from("Hello, world!");
    info!(format!("aad {}", 5));

    // macro_rules! my_println {
    //     ($arg:expr) => {
    //         println!("The argument is: {}\n", $arg);
    //     };
    // }
    // my_println!("rust");
}

