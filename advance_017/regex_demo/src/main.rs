use regex::Regex;

fn main() {}


/// 匹配手机号
#[test]
fn test_phone_regex() {
    let regex = Regex::new(r#"^1[0-9]{10}$"#).unwrap();
    let match_result = regex.is_match("16688889999");
    assert!(match_result);
}

/// 匹配邮箱
#[test]
fn test_email_regex() {
    let regex = Regex::new(r#"^[a-zA-Z0-9]{1,}@[a-zA-Z0-9]{1,}\.[a-zA-Z0-9]{1,}$"#).unwrap();
    let match_result = regex.is_match("rust@example.com");
    assert!(match_result);
}


#[test]
fn test_regex_find() {
    let regex = Regex::new(r#"1[0-9]{3}"#).unwrap();
    let haystack = "155,1888,1999,177,12,1666";
    let match_result = regex.find(haystack);
    println!("match_result: {:?}", match_result);
}

#[test]
fn test_regex_find_iter() {
    let regex = Regex::new(r#"1[0-9]{3}"#).unwrap();
    let haystack = "155,1888,1999,177,12,1666";
    for mat in regex.find_iter(haystack) {
        println!("match_result: {}", mat.as_str());
    }
}