
use crate::machine_learning::MachineLearning;

fn split_data(split_variable: String, split_value: f32) {
}

fn get_unique_values(variable_name: String) {
}


fn get_range(variable_name: String){
}





pub struct DecisionTree <T> {
  pub ml: MachineLearning<T>,
  pub criterion: String,
}

impl<T> DecisionTree<T>{
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

  pub fn get_unique_values(&self, variable_name: &str) {

    //variable_name.into()

    // &self.ml.data.entry(variable_name);

    // println!("{}", &self.ml.data);

  }


}
