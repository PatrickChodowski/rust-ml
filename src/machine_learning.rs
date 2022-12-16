

// Contains base trait for machine learning struct

use crate::utils::DataFrame;

pub struct MachineLearning<'a> {
  pub data: DataFrame<'a>, // Training data read from CSV
  pub target: String, // Name of target attribute
  pub index: String // Name of index attribute
}

impl <'a> MachineLearning<'a>{

  pub fn query_larger(&mut self, 
                where_column: &'a str, 
                where_value: &mut f32)  -> Vec<bool> {

    let mut res: Vec<bool> = Vec::new();
    let base_v = self.data.entry(where_column).or_default(); 
    let iter_v = base_v.into_iter();

    for a in iter_v {
      if a >= where_value {
        println!("{}", a);
        res.push(true);
      } else {
        res.push(false);
      }
    }

    return res;
  }

  pub fn query_smaller(&mut self, 
                       where_column: &'a str, 
                       where_value: &mut f32) -> Vec<bool> {
    let mut res: Vec<bool> = Vec::new();
    let base_v = self.data.entry(where_column).or_default(); 
    let iter_v = base_v.into_iter();

    for a in iter_v{
      if a < where_value {
        println!("{}", a);
        res.push(true);
      } else {
        res.push(false);
      }
    }

    return res;
  }

  pub fn count_targets(&'a mut self, query_v: &Vec<bool>) -> f32 {
    let mut total_targets: f32 = 0.0;
    let mut total_obs: f32 = 0.0;

    let targets_v = &self.data.entry(&self.target).or_default();
    let iter_query_v = query_v.iter();


    for (index, a) in iter_query_v.enumerate(){
      if *a {
        total_targets += targets_v[index];
        total_obs += 1.0;
      }
    }
    println!("Sum target positive: {} out of {}", total_targets, total_obs);

    return total_targets;
  }


}