
use core::cmp::Ordering::Equal;
use crate::machine_learning::MachineLearning;

fn split_data(split_variable: String, split_value: f32) {
}

pub fn check_if_boolean(unique_values: &mut Vec<f32>) -> bool {
  return unique_values.len() <= 2;
}



pub struct DecisionTree<'a> {
  pub ml: MachineLearning<'a>,
  pub criterion: String,
}

impl <'a> DecisionTree<'a>{
  pub fn info(&self){
    println!("Decision Tree. Criterion: {}; Index: {}; Target: {}", &self.criterion, &self.ml.index, &self.ml.target);
  }

  pub fn gini_impurity(&self){
    // Gini = 1 - sum(pi)^2
    todo!()
  }

  pub fn information_gain(&self){
    // Gini = 1 - sum(pi)^2
    todo!()
  }

  pub fn get_unique_values(&mut self, variable_name: &'a str) -> &mut Vec<f32> {

    println!("Getting unique values for {}", variable_name);
    let base_v = self.ml.data.entry(variable_name).or_default(); 
    base_v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    base_v.dedup();

    // let iter_v = base_v.into_iter();
    // for a in iter_v {
    //   println!("{}", a)
    // }
    println!("Found {} unique values for {}", base_v.len(), variable_name);
    return base_v;
  }





}
