fn main() {
    // 1. 字面量
    println!("****************1. 字面量***************");
    // （1）通过后缀表示类型
    let a = 7u8;
    let b = 5.4f64;
    let c = 32i32;
    let d = "zhangsan";

    // （2）通过前缀表示进制
    // 二进制
    let e = 0b1;
    // 八进制
    let f = 0o7;
    // 十六进制
    let g = 0xa;
    // byte
    let h = b'A';

    // （3）通过 “_” 来分割数据，易于可读性
    let j = 10_000;
    let k = 200_000.000_1;

    dbg!(a);
    dbg!(b);
    dbg!(c);
    dbg!(d);
    dbg!(e);
    dbg!(f);
    dbg!(g);
    dbg!(h);
    dbg!(j);
    dbg!(k);

    // 2. 算术运算符（1）
    println!("****************2. 算术运算符（1）***************");
    let aa = 12;
    let bb = 9;
    dbg!(aa + bb);
    dbg!(aa - bb);
    dbg!(aa * bb);
    dbg!(aa / bb);
    dbg!(aa % bb);
    // 不支持自定和自减运算符
    // aa++ bb++

    // 3. 算术运算符（2）
    println!("****************3. 算术运算符（2）***************");
    let mut cc = 20;
    let dd = 5;

    println!("初始值 cc = {} dd = {}", cc, dd);
    // cc = cc + dd;
    cc += dd;
    println!("经过 cc += dd 运算后，cc的值为:{:}", cc);
    // cc = cc - dd;
    cc -= dd;
    println!("经过 cc -= dd 运算后，cc的值为:{:}", cc);
    // cc = cc * dd;
    cc *= dd;
    println!("经过 cc *= dd 运算后，cc的值为:{:}", cc);
    // cc = cc / dd;
    cc /= dd;
    println!("经过 cc /= dd 运算后，cc的值为:{:}", cc);
    // cc = cc % dd;
    cc %= dd;
    println!("经过 cc %= dd 运算后，cc的值为:{:}", cc);

    // 自增
    cc += 1;
    println!("经过 cc += 1 运算后，cc的值为:{:}", cc);
    // 自减
    cc -= 1;
    println!("经过 cc -= 1 运算后，cc的值为:{:}", cc);

    // 4. 关系运算符
    println!("****************4. 关系运算符***************");
    let mm = 15;
    let nn = 30;

    dbg!(mm > nn);
    dbg!(mm < nn);
    dbg!(mm == nn);
    dbg!(mm >= nn);
    dbg!(mm <= nn);
    dbg!(mm != nn);

    // 5. 逻辑运算符
    println!("****************5. 逻辑运算符***************");
    let rr = 56;
    let ss = 34;

    dbg!(rr > 40 && ss > 40);
    dbg!(rr > 40 || ss > 40);
    dbg!(!(rr > 40));

    // 6. 位运算符
    println!("****************6. 位运算符***************");
    // 2 的 二进制表示
    let x: i8 = 0b00000010;
    // 7 的 二进制表示
    let y: i8 = 0b00000111;

    println!("{:08b} & {:08b} = {:08b}", x, y, x & y);
    println!("{:08b} | {:08b} = {:08b}", x, y, x | y);
    println!("{:08b} ^ {:08b} = {:08b}", x, y, x ^ y);
    println!("!{:08b} = {:08b}", x, !x);
    println!("{:08b} << 1 = {:08b}", x, x << 1);
    println!("{:08b} >> 1 = {:08b}", x, x >> 1);
}
