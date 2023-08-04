fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        min-height: 100px;
        min-width: 200px;
        Text {
            text: "hello slint";
            color: green;
            font-size: 30px;
        }
    }
}