mod machine_learning;
mod decision_tree;
mod utils;

use decision_tree::{DecisionTree, DTCritetion};
use machine_learning::{ML, MLType};
use utils::{read_csv, preprocess};


fn main() {

  let mut data = read_csv("./data/train.csv").ok().unwrap();
  data = preprocess(data);
  let mut ml = ML::new(&mut data, "Survived", "PassengerId", MLType::Classification);
  ml.set_colnames();


  let dt = DecisionTree::init(&ml, DTCritetion::Gini);
  dt.train();

  // println!("{}", ml.df);
  // println!("{}", ml.index);
  // println!("{}", ml.target);



}
