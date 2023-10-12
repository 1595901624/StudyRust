fn main() {
    println!("Hello, world!");
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[cfg(test)]
#[test]
fn test_fibonacci() {
    let x = fibonacci(10);
    println!("fib(10) = {}", x);
}
