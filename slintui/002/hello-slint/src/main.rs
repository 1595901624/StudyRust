slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // let ui = MarcoWindow::new()?;
    // ui.run()

    // MarcoWindow2::new().unwrap().run().unwrap();
    AppWindow::new().unwrap().run().unwrap();
    // AppWindow2::new().unwrap().run().unwrap();

    // MarcoWindow2::new().unwrap().show().unwrap();
    // AppWindow::new().unwrap().show().unwrap();
    Ok(())
}

// 通过 slint::slint! 宏来定义组件
slint::slint! {
    import { VerticalBox } from "std-widgets.slint";
    export component MarcoWindow inherits Window {
        VerticalBox { 
            Text {
                color: red;
                text: "Hello, world!";
            }
         }
     }
}

slint::slint! {
    import { VerticalBox } from "std-widgets.slint";
    export component MarcoWindow2 inherits Window {
        VerticalBox { 
            Text {
                color: green;
                text: "Hello, world!";
            }
         }
     }
}