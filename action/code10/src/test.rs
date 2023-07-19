#[test]
fn test_my_print() {





    macro_rules! my_println {
        ($arg:expr) => {
            println!("The argument is: {}\n", $arg);
        };
    }





    my_println!("rust");
}

#[test]
fn test_macro() {
    macro_rules! add {
        ($a:expr, $b:expr) => {
            $a + $b
        };
    }

    let sum = add!(5.0, 10.0);
    println!("sum = {}", sum);
}

