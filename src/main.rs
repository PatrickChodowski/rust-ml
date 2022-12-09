mod utils;
use utils::{read_data, print_data};


fn main() {
  // If there is no data it might as well fail
  // let res = read_data("./data/train2.csv").unwrap();

  // Handle result properly
  let res_data = read_data("./data/train.csv");
  if let Ok(e) = res_data {
    print_data(e, 3);
  } else if let Err(e) = res_data {
    eprintln!("{}", e);
  }

}
