use std::ffi::c_char;

fn main() {
    println!("Hello, world!");
}

#[link(name = "clua_5_4_6", kind = "static")]
extern "C" {
    fn execute(code: *const c_char);
}