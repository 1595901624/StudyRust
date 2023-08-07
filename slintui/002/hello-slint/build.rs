fn main() {
    println!("cargo:warning============= compile build.rs =============\n");
    slint_build::compile("ui/appwindow.slint").unwrap();
    // slint_build::compile("ui/appwindow2.slint").unwrap();
}

