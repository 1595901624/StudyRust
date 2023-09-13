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
}
