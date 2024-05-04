mod test;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  test::test001::_test004(&args);
}
