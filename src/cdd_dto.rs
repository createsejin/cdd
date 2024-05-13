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
}
