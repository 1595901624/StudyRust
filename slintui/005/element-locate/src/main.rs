fn main() {
    App::new().unwrap().run().unwrap();
}
slint::slint! {
    // 显示定位
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

    // 百分比
    component PercentView inherits Rectangle {
        width: 300px;
        height: 300px;
        background: red;
        Rectangle {
            // 默认不标注 x 和 y，表示居中
            // x: 0;
            // y: 0;
            // 支持百分比
            width: 50%;
            height: 50%;
            background: green;
        }
    }

    // 首选尺寸
    component PreferredView inherits Rectangle {
        preferred-height: 300px;
        preferred-width: 300px;
        background: purple;

        rec := Rectangle {
            // x: 0;
            // y: 0;
            width: 100px;
            height: 100px;
            background: green;
        }
    }
}
