use std::env;
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
    print!("len={}|", args.len());
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

pub fn _test005() {
  let args: Vec<String> = env::args().collect();
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

#[allow(dead_code)]
fn read_cdd_data(path: &str) -> Vec<(u8, String, String, String, String)> {
  let file = File::open(path.to_string()).expect("file not found \u{25A1}");
  let reader = io::BufReader::new(file);

  let mut rows: Vec<(u8, String, String, String, String)> = Vec::new();
  for line in reader.lines() {
    let line_str = line.unwrap();
    let args_str: Vec<&str> = line_str.split('|').collect();
    let argn = args_str.len() as u8;
    assert!(argn >= 2, "not enough arguments!");
    let arg1 = args_str[0].to_string();
    let arg2 = args_str[1].to_string();
    let arg3 = if argn >= 3 {
      args_str[2].to_string()
    } else {
      String::from("")
    };
    let arg4 = if argn >= 4 {
      args_str[3].to_string()
    } else {
      String::from("")
    };
    assert!(argn < 5, "too many arguments!");
    let row = (argn, arg1, arg2, arg3, arg4);
    rows.push(row);
  }
  rows
}

#[allow(dead_code)]
fn print_rows(rows: &[(u8, String, String, String, String)]) {
  for row in rows {
    let (num, arg1, arg2, arg3, arg4) = row;
    let mut row_str = format!("{num}|{arg1}|{arg2}");
    if *num >= 3 {
      row_str.push('|');
      row_str.push_str(arg3);
    }
    if *num >= 4 {
      row_str.push('|');
      row_str.push_str(arg4);
    }
    println!("{row_str}");
  }
}

#[test]
pub fn _test007() -> io::Result<()> {
  let rows = read_cdd_data("cdd_data.txt");
  print_rows(&rows);
  Ok(())
}
