use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    // js 函数
    pub fn alert(msg: &str);
}

#[wasm_bindgen]
pub fn alert_by_rust(msg: &str) {
    return alert(msg);
}


/// 调用 setTimeout
#[wasm_bindgen]
pub fn test_setTimeout() {
    // 声明一个函数
    let func = Function::new_no_args(r#"alert("hello wasm")"#);
    // 获取 window
    let window = window().unwrap();
    // window 调用setimeout
    window.set_timeout_with_callback_and_timeout_and_arguments_0(&func, 1000);
}


/// 操作Dom
#[wasm_bindgen]
pub fn dom() {
    // 获取window
    let window = window().unwrap();
    // 获取document
    let doc = window.document().unwrap();
    // 获取test节点（我在html声明了一个div）
    let test_node = doc.get_element_by_id("test").unwrap();
    // 在节点里添加内容
    test_node.set_text_content(Some("Rust 操作 Dom"));

    // 最后再来个alert
    window.alert_with_message("我是通过 web_sys 生成的");
}