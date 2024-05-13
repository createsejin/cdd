use std::cmp::Ordering;

#[derive(Clone, Eq)]
pub struct Dto {
  args: Vec<String>,
  dir: String,
}

impl Dto {
  pub fn new(args: Vec<String>, dir: String) -> Self {
    Self { args, dir }
  }

  #[allow(dead_code)]
  pub fn print_row(&self) {
    let mut print_str = String::new();
    self
      .args
      .iter()
      .for_each(|s| print_str.push_str(format!("{s}|").as_str()));
    print_str.push_str(self.dir.as_str());
    println!("{print_str}");
  }

  #[allow(dead_code)]
  pub fn get_args(&self) -> &[String] {
    &self.args
  }
}

impl PartialEq for Dto {
  fn eq(&self, other: &Self) -> bool {
    self.args == other.args
  }
}

impl PartialOrd for Dto {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}

impl Ord for Dto {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let first_arg_cmp = self.args[0].cmp(&other.args[0]);
    if first_arg_cmp == Ordering::Equal {
      let argn_cmp = self.args.len().cmp(&other.args.len());
      if argn_cmp == Ordering::Equal {
        if self.args.len() >= 2 && other.args.len() >= 2 {
          let second_arg_cmp = self.args[1].cmp(&other.args[1]);
          if second_arg_cmp == Ordering::Equal {
            if self.args.len() >= 3 && other.args.len() >= 3 {
              // if argn is 3, cmp third arg
              self.args[2].cmp(&other.args[2])
            } else {
              // if not argn is over or equal 3
              second_arg_cmp
            }
          } else {
            // if not result of second_arg_cmp is Equal
            second_arg_cmp
          }
        } else {
          // if not argn is over or equal 2
          argn_cmp
        }
      } else {
        // if not result of argn_cmp is Equal
        argn_cmp
      }
    } else {
      // if not result of first_arg_cmp is Equal
      first_arg_cmp
    }
  }
}
