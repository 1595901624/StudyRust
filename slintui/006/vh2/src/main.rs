fn main() {
    println!("Hello, world!");
    App::new().unwrap().run().unwrap();
}

slint::slint! {
    export component App inherits Window {
        width: 400px;
        height: 400px;
    }

    component HorizontalStretchComponment inherits Rectangle {
        width: 500px;
        height: 500px;

        HorizontalLayout {

            Rectangle {
                horizontal-stretch: 0;
                min-width: 100px;
                background: red;
            }
            Rectangle {
                horizontal-stretch: 1;
                min-width: 200px;
                background: green;
            }
            Rectangle {
                horizontal-stretch: 2;
                min-width: 100px;
                background: blue;
            }
        }
    }
}
