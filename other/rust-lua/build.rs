const CARGO_RUSTC_LINK_SEARCH_NATIVE_C: &str = "cargo:rustc-link-search=native=./c";
const CARGO_RUSTC_LINK_LIB_STATIC_LUA: &str = "cargo:rustc-link-lib=static=lua";
const CARGO_RUSTC_LINK_SEARCH_NATIVE_LUA_5_4_6: &str = "cargo:rustc-link-search=native=./lua";
// const CARGO_RUSTC_LINK_SEARCH_NATIVE_LUA_5_4_0: &str = "cargo:rustc-link-search=native=./lua-5.4.0";
const CARGO_RUSTC_LINK_LIB_STATIC_CLUA_5_4_6: &str = "cargo:rustc-link-lib=static=clua_5_4_6";
// const CARGO_RUSTC_LINK_LIB_STATIC_CLUA_5_4_0: &str = "cargo:rustc-link-lib=static=clua_5_4_0";

fn main() {
    println!("{}", CARGO_RUSTC_LINK_SEARCH_NATIVE_C);
    println!("{}", CARGO_RUSTC_LINK_LIB_STATIC_LUA);
    println!("{}", CARGO_RUSTC_LINK_SEARCH_NATIVE_LUA_5_4_6);
    // println!("{}", CARGO_RUSTC_LINK_SEARCH_NATIVE_LUA_5_4_0);
    println!("{}", CARGO_RUSTC_LINK_LIB_STATIC_CLUA_5_4_6);
    // println!("{}", CARGO_RUSTC_LINK_LIB_STATIC_CLUA_5_4_0);
}