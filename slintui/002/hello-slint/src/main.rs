slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 运行 通过 slint::slint! 宏来定义组件
    MarcoWindow::new().unwrap().run().unwrap();
    // MarcoWindow2::new().unwrap().run().unwrap();
    // 运行 通过 slint_build::compile 生成的组件
    // AppWindow::new().unwrap().run().unwrap();
    Ok(())
}

// 通过 slint::slint! 宏来定义组件1
slint::slint! {
    import { VerticalBox } from "std-widgets.slint";
    export component MarcoWindow inherits Window {
        min-height: 100px;
        min-width: 200px;
        VerticalBox { 
            Text {
                color: red;
                text: "Hello, world! rust!!";
            }
         }
     }
}

// 通过 slint::slint! 宏来定义组件2
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
