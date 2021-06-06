use std::process::Command;

fn main() {
   println!("hello world~~");

   // 命令提示符 pause
   let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();	
}