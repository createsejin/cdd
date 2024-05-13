use core::panic;
use std::env;

#[allow(dead_code)]
pub struct ArgmentPaser<'a> {
  args: Vec<String>,
  argn: u8,
  // rows is vector ref of external struct. so it need lifetime parameter
  // lifetime of rows ref should be equal to this structure
  rows: &'a [(u8, String, String, String, String)],
  dir: Vec<String>,
}

#[allow(dead_code)]
impl<'a> ArgmentPaser<'a> {
  pub fn new(rows: &'a [(u8, String, String, String, String)]) -> Self {
    Self {
      args: Vec::new(),
      argn: 0,
      rows,
      dir: Vec::new(),
    }
  }

  pub fn collect_args(&mut self) {
    self.args = env::args().collect();
    self.argn = self.args.len() as u8;
    // self._args_test_print();
  }

  pub fn parse_argment(&mut self) {
    if self.args[1] == *"-a" {
      println!("first='-a', argn={}", self.argn);
    } else if self.args[1] == *"-cd" {
      self.loop_rows();
    }
  }

  fn loop_rows(&mut self) {
    for row in self.rows {
      if row.0 == (self.argn - 1) {
        match self.argn {
          3 => {
            // cddr -cd se => argn=3
            if self.args[2] == row.1 {
              self.dir.push(row.2.clone());
            }
          }
          4 => {
            // cddr -cd se . => argn=4
            if self.args[2] == row.1 && self.args[3] == row.2 {
              self.dir.push(row.3.clone());
            }
          }
          5 => {
            // cddr -cd se . a => argn=5
            if self.args[2] == row.1 && self.args[3] == row.2 && self.args[4] == row.3 {
              self.dir.push(row.4.clone());
            }
          }
          _ => panic!("cdd_data.txt have some invaild row \u{25A1}"),
        }
      }
    }
    assert_eq!(self.dir.len(), 1, "result dir is not one!");
    println!("{}", self.dir[0]);
  }

  fn _args_test_print(&self) {
    println!("argn = {}", self.argn);
    for (i, arg) in self.args.iter().enumerate() {
      if i == (self.argn - 1).into() {
        // convert u8 to usize to compare i
        print!("{}", *arg);
      } else {
        print!("{}|", *arg);
      }
    }
    println!();
  }
}

// #[allow(dead_code)]
// pub fn initialize() {
//   let mut file_manager = FileManager::new("cdd_data.txt");
//   file_manager.read_cdd_data().unwrap();
//
//   let mut argment_parser = ArgmentPaser::new(file_manager.get_rows());
//   argment_parser.collect_args();
//   argment_parser.parse_argment();
// }
