mod machine_learning;
mod decision_tree;
mod utils;

use decision_tree::{DecisionTree, DTCritetion};
use machine_learning::{ML, MLType};
use utils::{read_csv, preprocess};


fn main() {
  // if there is no data it might as well fail immediately
  let mut data = read_csv("./data/train.csv").ok().unwrap();
  data = preprocess(data);
  let ml = ML::new(&mut data, "Survived", "PassengerId", MLType::Classification);
  let dt = DecisionTree::init(&ml, DTCritetion::Gini);
  dt.train();

  // println!("{}", ml.df);
  // println!("{}", ml.index);
  // println!("{}", ml.target);



}
