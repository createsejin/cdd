use super::cdd_dto::Dto;
use std::env;

#[allow(dead_code)]
pub struct ArgumentPaser<'a> {
  args: Vec<String>,
  argn: usize,
  rows: &'a [Dto],
}

#[allow(dead_code)]
impl<'a> ArgumentPaser<'a> {
  pub fn new(rows: &'a [Dto]) -> Self {
    Self {
      args: Vec::new(),
      argn: 0,
      rows,
    }
  }

  pub fn collect_args(&mut self) {
    self.args = env::args().collect();
    self.argn = self.args.len();
    self._args_test_print();
  }

  pub fn parse_argument(&mut self) {
    if self.args[1] == *"-cd" {}
  }

  fn _args_test_print(&self) {
    println!("argn = {}", self.argn);
    for (i, arg) in self.args.iter().enumerate() {
      if i == self.argn - 1 {
        // convert u8 to usize to compare i
        print!("{}", *arg);
      } else {
        print!("{}|", *arg);
      }
    }
    println!();
  }
}
