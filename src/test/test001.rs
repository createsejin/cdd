use std::io::{self, Write};
use std::process::Command;

pub fn _test001() {
  let output = Command::new("sh")
    .arg("-c")
    .arg("echo hello")
    .output()
    .expect("failed to execute process");

  let hello = String::from_utf8_lossy(&output.stdout);
  println!("{hello}");
}

pub fn _test002() {
  let eza_output = Command::new("sh")
    .arg("-c")
    .arg("eza -A --long --color=always --icons=always --git")
    .output()
    .expect("failed to execute process");

  // get output String from eza_output.stdout
  // let output = String::from_utf8_lossy(&eza_output.stdout);
  // println!("{output}");

  io::stdout().write_all(&eza_output.stdout).unwrap();
  io::stderr().write_all(&eza_output.stdout).unwrap();

  println!("status: {}", eza_output.status);
  assert!(eza_output.status.success());
}
