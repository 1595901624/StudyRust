fn main() {
    App::new().unwrap().run().unwrap();
}
slint::slint! {
    export component App inherits Window {
        width: 300px;
        height: 300px;
        background: red;
        Rectangle {
            x: 100px;
            y: 100px;
            background: green;

            Rectangle {
                x: 100px;
                y: 100px;
                background: yellow;
            }
        }
    }
}
