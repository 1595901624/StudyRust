#[test]
fn test_type() {
    type Int = i32;
    type Float = f32;
    type Double = f64;
    type Char = char;

    let a: Int = 3;
    let b: Float = 4.5;
    let c: Double = 134.6753453424234231;
    let d: Char = '我';

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
}

#[test]
fn test_cfg_macro() {
    if cfg!(target_os = "windows") {
        println!("hello windows!");
    } else if cfg!(target_os = "linux") {
        println!("hello linux!");
    }
}

#[test]
fn test_cfg_property() {
    print();

    #[cfg(target_os = "windows")]
    fn print() {
        println!("hello windows!");
    }

    #[cfg(target_os = "linux")]
    fn print() {
        println!("hello linux!");
    }
}