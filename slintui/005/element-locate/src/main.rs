fn main() {
    App::new().unwrap().run().unwrap();
}
slint::slint! {
    // 自动定位
    export component App inherits Window {
        width: 300px;
        height: 300px;
        background: red;
      
    }

}
