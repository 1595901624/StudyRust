fn main() {
    println!("Hello, world!");

    unsafe {
        let x = "rust".to_string();
        x.get_unchecked()
    }
}

