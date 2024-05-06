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
  rows.sort();
  rows
}

#[allow(dead_code)]
fn print_rows<'a, T>(rows: &'a T)
where
  &'a T: IntoIterator<Item = &'a (u8, String, String, String, String)>,
{
  for row in rows {
    let row_str = _format_row(row);
    println!("{row_str}");
  }
}

fn _format_row(row: &(u8, String, String, String, String)) -> String {
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
  row_str
}

fn _remove_dir(
  rows: &[(u8, String, String, String, String)],
) -> Vec<(u8, String, String, String, String)> {
  let mut dir_removed_rows: Vec<(u8, String, String, String, String)> = Vec::new();
  for row in rows {
    let mut cloned_row = row.clone();
    let argn = cloned_row.0;
    match argn {
      2 => cloned_row.2 = String::from(""),
      3 => cloned_row.3 = String::from(""),
      4 => cloned_row.4 = String::from(""),
      _ => (),
    }
    dir_removed_rows.push(cloned_row);
  }
  dir_removed_rows
}

#[allow(dead_code)]
fn detact_ded_row(rows: &[(u8, String, String, String, String)]) {
  // dir is removed to compare duplicated rows
  let mut dir_removed_rows = _remove_dir(rows);
  print_rows(&dir_removed_rows);
  // clone dir_removed_rows and make it deduped_rows
  let mut deduped_rows = dir_removed_rows.clone();
  deduped_rows.dedup();
  println!("\ndeduped: ");
  print_rows(&dir_removed_rows);
  println!();
  // collection of duplicate_row
  let mut duplicate_row: Vec<(u8, String, String, String, String)> = Vec::new();
  // iterating over until deduped_rows has been clean
  while deduped_rows.len() > 0 {
    // duplication check: if compared rows is equal, both rows is removed.
    if dir_removed_rows[0] == deduped_rows[0] {
      dir_removed_rows.remove(0);
      deduped_rows.remove(0);
    // if compare rows is not equal,
    } else {
      // pushed duplicate row to duplicate_row
      duplicate_row.push(dir_removed_rows[0].clone());
      // and remove row of dir_removed_rows only
      dir_removed_rows.remove(0);
    } // by this method, we can compare same elements of both rows
  }

  println!("duplicate_row:");
  print_rows(&duplicate_row);
}

#[test]
fn _test007() {
  let rows = read_cdd_data("cdd_data.txt");
  detact_ded_row(&rows);
}
