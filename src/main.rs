use std::fs;
use std::env;
use std::collections::HashMap;

static TESTING: bool = true;


fn print_warning() {
  let mut location = String::from("data/warning.txt");
  if TESTING {
    location = String::from("/home/runner/SimpleCrypt/src/") + &location;
  }
  let warning = fs::read_to_string(location).expect("File 'data/warning.txt' should come preinstalled.");
  println!("{}", warning);
}


fn get_args() -> String {
  let args: Vec<String> = env::args().collect();
  let mode = args.get(1);
  dbg!(mode);
  return String::from("mode")
}


  
fn main() {
  let args = get_args();
  
  print_warning();
}