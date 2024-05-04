use std::fs::File;
use std::io::{self, BufRead, Write};
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

#[allow(dead_code)]
fn get_input(prompt: String) -> String {
  print!("{}", prompt);
  io::stdout().flush().unwrap();

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input.trim().to_string()
}

// test read lines
pub fn _test003() -> std::io::Result<()> {
  let f = File::open("cdd_data.txt").expect("file not found");
  let reader = io::BufReader::new(f);

  // collect lines into Vec<Vec<String>>
  // inside Vec<String> is a args
  // outside Vec<Vec<String>> is a lines
  let mut lines: Vec<Vec<String>> = Vec::new();

  // read lines
  for line in reader.lines() {
    // unwrap line
    let line_str = line.unwrap();
    // split line by '|'
    let args_str: Vec<&str> = line_str.split('|').collect();
    // convert Vec<&str> to Vec<String>
    let args_string: Vec<String> = args_str.iter().map(|x| x.to_string()).collect();
    // push args_string: Vec<String> to lines: Vec<Vec<String>>
    lines.push(args_string);
  }

  // test print lines
  for args in lines {
    for (i, arg) in args.iter().enumerate() {
      // if i is the last index of args
      if i == args.len() - 1 {
        // print arg without '|'
        print!("{}", *arg);
      } else {
        // if i is not the last index of args
        // print arg with '|'
        print!("{}|", *arg);
      }
    }
    // after print all args, print new line
    println!();
  }

  Ok(())
}

pub fn _test004(args: &[String]) {
  let argn = args.len();
  println!("argn = {}", argn);
  for (i, arg) in args.iter().enumerate() {
    if i == argn - 1 {
      print!("{}", *arg);
    } else {
      print!("{}|", *arg);
    }
  }
  println!();
}
