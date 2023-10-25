use regex::Regex;

fn main() {

    // 匹配手机号
    let regex = Regex::new(r#"^1[0-9]{10}$"#).unwrap();
    let match_result = regex.is_match("16688889999");
    println!("{}", match_result);
}
