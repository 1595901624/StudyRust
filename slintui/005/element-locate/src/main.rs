fn main() {
    App::new().unwrap().run().unwrap();
}
slint::slint! {
    // 自动定位
    import { Button } from "std-widgets.slint";
export component App inherits Window {
        width: 300px;
        height: 300px;
        background: red;

    }

    // 垂直布局
    component VerticalLayoutTest {
        width: 300px;
        height: 300px;

        VerticalLayout {
            width: 300px;
            height: 300px;

            Rectangle {
                height: 100px;
                background: blue;
            }

            Rectangle {
                height: 100px;
                background: red;
            }

            Rectangle {
                height: 100px;
                background: green;
            }
        }
    }

    // 水平布局
    component HorizontalLayoutTest {
        width: 300px;
        height: 300px;

        HorizontalLayout {
            width: 300px;
            height: 300px;

            Rectangle {
                width: 100px;
                background: blue;
            }

            Rectangle {
                width: 100px;
                background: red;
            }

            Rectangle {
                width: 100px;
                background: green;
            }
        }
    }

}
