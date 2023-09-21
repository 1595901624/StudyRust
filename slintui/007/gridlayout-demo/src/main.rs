use slint::slint;

fn main() {
    println!("Hello, world!");
}

slint! {
    export component App inherits Window {
        
    }
    // 网格布局
    component GridLayoutTest {
        width: 300px;
        height: 300px;

        GridLayout {
            width: 300px;
            height: 300px;

            Rectangle {
                width: 50px;
                height: 300px;
                background: blue;
            }
            Rectangle {
                width: 50px;
                height: 300px;
                background: red;
            }
            Rectangle {
                width: 50px;
                height: 300px;
                background: green;
            }
            Rectangle {
                width: 50px;
                height: 300px;
                background: blue;
            }
            Rectangle {
                width: 50px;
                height: 300px;
                background: red;
            }
            Rectangle {
                width: 50px;
                height: 300px;
                background: green;
            }
        }
    }

    // Row元素
    component RowElementest {
        GridLayout {
            width: 300px;
            height: 300px;

            Row {
                Rectangle {
                    background: red;
                }
                Rectangle {
                    background: green;
                }
            }

            Row {
                Rectangle {
                    background: cyan;
                }
                Rectangle {
                    background: orange;
                }
            }
        }
    }
}
