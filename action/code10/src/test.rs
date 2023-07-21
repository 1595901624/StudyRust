#[test]
fn test_my_print() {
    macro_rules! my_println {
        ($arg:expr) => {
            println!("The argument is: {}\n", $arg);
        };
    }
    my_println!("rust");
}

/// 重复
#[test]
fn test_repeat() {
    macro_rules! repeat {
        ($($value:expr),*) => {
        {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($value);
                )* // 这里的 * 表示重复 0 到多次
                temp_vec
            }
        };
    }
    let vec = repeat!(1,2,3,4,5);
    println!("vec = {:?}", vec);
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


