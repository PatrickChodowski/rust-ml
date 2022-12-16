

// Contains base trait for machine learning struct

use crate::utils::DataFrame;

pub struct MachineLearning<'a> {
  pub data: DataFrame<'a>, // Training data read from CSV
  pub target: String, // Name of target attribute
  pub index: String // Name of index attribute
}

impl <'a> MachineLearning<'a>{

  pub fn query(&mut self, 
                where_column: &'a str, 
                where_value: &mut f32, 
                where_operator: &str) {

    let base_v = self.data.entry(where_column).or_default(); 
    let iter_v = base_v.into_iter();

    for a in iter_v {
      if a >= where_value {
        println!("{}", a)
      }
    }
  }

}