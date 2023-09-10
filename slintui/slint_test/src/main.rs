fn main() {
    println!("Hello, world!");
    App::new().unwrap().run().unwrap();
}

slint::slint! {
    import { StandardButton, Button } from "std-widgets.slint";

    export component App inherits Window {
        width: 300px;
        height: 300px;

        popup := PopupWindow {
            Rectangle { height:100%; width: 100%; background: blue; }
            x: 20px; y: 20px; height: 50px; width: 50px;
        }

        TouchArea {
            height:100%; width: 100%;
            clicked => { popup.show(); }
        }
    }

    export component Example inherits Dialog {
        Text {
            text: "This is a dialog box";
        }
        StandardButton { kind: ok; }
        StandardButton { kind: cancel; }
        Button {
        text: "More Info";
        dialog-button-role: action;
    }
}
}
