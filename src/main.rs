mod machine_learning;
mod decision_tree;
mod utils;

use decision_tree::DecisionTree;
use machine_learning::MachineLearning;
use utils::{read_data3, DataFrame};


fn main() {
  
  let res_data3: DataFrame = read_data3("./data/final_train.csv").unwrap();
  // initialize decision tree

  let dt: DecisionTree<DataFrame> = DecisionTree{
    ml: MachineLearning{data: res_data3, target: "survived".into(), index: "passenger_id".into()},
    criterion: "gini".into()
  };

  dt.info();
  dt.get_unique_values("fare".into());


}
