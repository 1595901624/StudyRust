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


    // 对齐
    export component AlignmentTest {
        width: 400px;
        height: 400px;
        VerticalLayout {
            HorizontalLayout {
                alignment: stretch;
                Rectangle { background: red; min-width: 10px; }
                Rectangle { background: blue; min-width: 20px; }
                Rectangle { background: green; min-width: 30px; }
            }
            HorizontalLayout {
                alignment: start;
                Rectangle { background: red; min-width: 10px; }
                Rectangle { background: blue; min-width: 20px; }
                Rectangle { background: green; min-width: 30px; }
            }
            HorizontalLayout {
                alignment: center;
                Rectangle { background: red; min-width: 10px; }
                Rectangle { background: blue; min-width: 20px; }
                Rectangle { background: green; min-width: 30px; }
            }
            HorizontalLayout {
                alignment: end;
                Rectangle { background: red; min-width: 10px; }
                Rectangle { background: blue; min-width: 20px; }
                Rectangle { background: green; min-width: 30px; }
            }
            HorizontalLayout {
                alignment: space-between;
                Rectangle { background: red; min-width: 10px; }
                Rectangle { background: blue; min-width: 20px; }
                Rectangle { background: green; min-width: 30px; }
            }
            HorizontalLayout {
                alignment: space-around;
                Rectangle { background: red; min-width: 10px; }
                Rectangle { background: blue; min-width: 20px; }
                Rectangle { background: green; min-width: 30px; }
            }
        }
    }

    export component ExportedComponentTest {
        width: 400px;
        height: 400px;

        VerticalLayout {
            alignment: end;
            Rectangle { background: red; min-height: 10px; }
            Rectangle { background: blue; min-height: 20px; }
            Rectangle { background: green; min-height: 30px; }
        }
        
    }
}
