fn main() {
    println!("Hello, world!");

    let x = X { name: "hello".to_string() };
    println!("{}", x.name);

}

struct X {
    name: String
}
