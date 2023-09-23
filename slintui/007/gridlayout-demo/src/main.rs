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
        width: 300px;
        height: 300px;

        GridLayout {
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

    // 属性
    component AttriibuteTest {
        width: 300px;
        height: 300px;

        GridLayout {
            rec1 := Rectangle {
                row: 0;
                col: 1;
                rowspan: 2;
                background: red;
            }
            rec2 := Rectangle {
                row: 1;
                background: green;
            }
            rec3 := Rectangle {
                row: 2;
                colspan: 2;
                background: blue;
            }
        }
    }

    // 嵌套布局
    component NestedLayoutTest {
        width: 300px;
        height: 300px;

        GridLayout {
            VerticalLayout {
                row: 0;
                Rectangle {
                    background: red;
                }
                Rectangle {
                    background: blue;
                }
            }
            HorizontalLayout {
                row: 1;
                Rectangle {
                    background: green;
                }
                Rectangle {
                    background: yellow;
                }
            }
        }
    }
}
