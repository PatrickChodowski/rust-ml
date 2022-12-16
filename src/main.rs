mod machine_learning;
mod decision_tree;
mod utils;

use decision_tree::{DecisionTree, check_if_boolean};
use machine_learning::MachineLearning;
use utils::{read_data_to_df, DataFrame};


fn main() {
  
  let data: DataFrame = read_data_to_df("./data/final_train.csv").unwrap();
  // initialize decision tree

  let mut dt: DecisionTree = DecisionTree{
    ml: MachineLearning{data: data, 
                        target: "survived".into(), 
                        index: "passenger_id".into()},
    criterion: "gini".into()
  };

  dt.info();
  let fare_values: &mut Vec<f32> = dt.get_unique_values("fare".into());
  check_if_boolean(fare_values);

  let v_larger = dt.ml.query_larger("fare", &mut 0.0);
  let v_smaller = dt.ml.query_smaller("fare", &mut 0.0);

  dt.ml.count_targets(&v_larger);
  // dt.ml.count_tarsgets(&v_smaller);

}
