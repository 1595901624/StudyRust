fn main() {
    // slint_build::compile("ui/appwindow.slint").unwrap();
    // slint_build::compile("ui/appwindow2.slint").unwrap();
    println!("cargo:warning================================== compile build.rs =================================\n");
    if let Ok(_) = slint_build::compile("ui/appwindow.slint") {
        println!("cargo:warning=compile ui/appwindow.slint success\n");
    }
    if let Ok(_) = slint_build::compile("ui/appwindow2.slint") {
        println!("cargo:warning=compile ui/appwindow2.slint success\n");
    }
}

