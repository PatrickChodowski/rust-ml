// mod machine_learning;
// mod decision_tree;
mod utils;

// use decision_tree::{DecisionTree, check_if_boolean};
// use machine_learning::MachineLearning;
use utils::{read_csv, preprocess};


fn main() {
  // if there is no data it might as well fail immediately
  let mut data = read_csv("./data/train.csv").ok().unwrap();
  data = preprocess(data);

  println!("data: {:?}", data);
  println!("columns: {:?}", data.get_column_names());





  // let mut dt: DecisionTree = DecisionTree{
  //   ml: MachineLearning{data: data, 
  //                       target: "survived".into(), 
  //                       index: "passenger_id".into()},
  //   criterion: "gini".into()
  // };

  // dt.info();
  // let fare_values: &mut Vec<f32> = dt.get_unique_values("fare".into());
  // check_if_boolean(fare_values);

  // let v_larger = dt.ml.query_larger("fare", &mut 0.0);
  // let v_smaller = dt.ml.query_smaller("fare", &mut 0.0);

  // dt.ml.count_targets(&v_larger);
  // // dt.ml.count_tarsgets(&v_smaller);

}
