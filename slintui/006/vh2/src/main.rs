fn main() {
    let app = App::new().unwrap();

    let blue_rect_width = app.get_blue_rect_width();
    let red_rect_width = app.get_red_rect_width();
    let green_rect_width = app.get_green_rect_width();
    let orange_rect_width = app.get_orange_rect_width();
    let yellow_rect_width = app.get_yellow_rect_width();
    let pink_rect_width = app.get_pink_rect_width();

    println!("blue_rect_width: {}", blue_rect_width);
    println!("red_rect_width: {}", red_rect_width);
    println!("green_rect_width: {}", green_rect_width);
    println!("orange_rect_width: {}", orange_rect_width);
    println!("yellow_rect_width: {}", yellow_rect_width);
    println!("pink_rect_width: {}", pink_rect_width);

    app.run().unwrap()
}

slint::slint! {
    // stretch demo1
    export component App inherits Window {

        out property <length> blue_rect_width: blue-rect.width;
        out property <length> red_rect_width: red-rect.width;
        out property <length> green_rect_width: green-rect.width;
        out property <length> orange_rect_width: orange-rect.width;
        out property <length> yellow_rect_width: yellow-rect.width;
        out property <length> pink_rect_width: pink-rect.width;

        width: 300px;
        height: 300px;

        HorizontalLayout {
            blue-rect := Rectangle {
                background: blue;
                min-width: 100px;
                horizontal-stretch: 0;
            }
            red-rect := Rectangle {
                width: 50px;
                background: red;
            }
            green-rect := Rectangle {
                background: green;
                max-width: 20px;
                horizontal-stretch: 2;
            }
            orange-rect := Rectangle {
                background: orange;
                width: 10px;
                horizontal-stretch: 8;
            }
            yellow-rect := Rectangle {
                background: yellow;
                max-width: 200px;
                horizontal-stretch: 1;
            }
            pink-rect := Rectangle {
                background: pink;
                horizontal-stretch: 2;
                max-height: 9000px;
            }

        }

    }

    // 拉伸demo2
    component StretchExample2 inherits Window {
        width: 300px;
        height: 200px;
        VerticalLayout {
            // 相同的拉伸因子（默认为1）：尺寸平均分配
            hl1 := HorizontalLayout {
                Rectangle { background: blue; }
                Rectangle { background: yellow;}
                Rectangle { background: green;}
            }
            // 在元素展开之前，具有较大最小宽度的元素会被分配更大的尺寸。
            hl2 := HorizontalLayout {
                Rectangle { background: cyan; min-width: 100px;}
                Rectangle { background: magenta; min-width: 50px;}
                Rectangle { background: gold;}
            }
            // 拉伸因子增加一倍：增长的幅度也增加一倍。
            hl3 := HorizontalLayout {
                Rectangle { background: navy; horizontal-stretch: 2;}
                Rectangle { background: gray; }
            }
            // 所有没有最大宽度限制的元素的拉伸因子为0，因此它们会自由地增长。
            hl4 := HorizontalLayout {
                Rectangle { background: red; max-width: 20px; }
                Rectangle { background: orange; horizontal-stretch: 0; }
                Rectangle { background: pink; horizontal-stretch: 0; }
            }
        }
    }

    // for
    export component ForExample {
        width: 200px;
        height: 150px;
        HorizontalLayout {
            Rectangle { background: red; }
            for t in [ "Hello", "Rust", "Slint" ] : Text {
                text: t;
            }
            Rectangle { background: cyan; }
        }
    }
}
