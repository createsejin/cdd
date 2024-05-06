use std::fs::File;
use std::io::{self, BufRead};

fn read_cdd_data(path: &str) -> Vec<(u32, u8, String, String, String, String)> {
  let file = File::open(path.to_string()).expect("file not found \u{25A1}");
  let reader = io::BufReader::new(file);

  let mut rows: Vec<(u32, u8, String, String, String, String)> = Vec::new();
  for line in reader.lines() {
    let line_str = line.unwrap();
    // split and collect args in data
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
    let row = (0, argn, arg1, arg2, arg3, arg4);
    rows.push(row);
  }
  // sort order: arg1, argn, arg2, arg3
  rows.sort_by(|a, b| {
    let arg1_cmp = a.2.cmp(&b.2);
    if arg1_cmp == std::cmp::Ordering::Equal {
      let argn_cmp = a.1.cmp(&b.1);
      if argn_cmp == std::cmp::Ordering::Equal {
        let arg2_cmp = a.3.cmp(&b.3);
        if arg2_cmp == std::cmp::Ordering::Equal {
          a.4.cmp(&b.4) // fourth
        } else {
          arg2_cmp // third
        }
      } else {
        argn_cmp // second
      }
    } else {
      arg1_cmp // first
    }
  });
  rows
}

fn _print_rows(rows: &[(u32, u8, String, String, String, String)]) {
  for row in rows {
    let (id, argn, arg1, arg2, arg3, arg4) = row;
    let mut row_str = format!("{id}|{argn}|{arg1}|{arg2}");
    if *argn >= 3 {
      row_str.push('|');
      row_str.push_str(arg3);
    }
    if *argn >= 4 {
      row_str.push('|');
      row_str.push_str(arg4);
    }
    println!("{row_str}");
  }
}

#[test]
fn _test010() {
  let rows = read_cdd_data("cdd_data.txt");
  _print_rows(&rows);
}
