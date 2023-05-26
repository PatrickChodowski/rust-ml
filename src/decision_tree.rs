
// use core::cmp::Ordering::Equal;
use crate::machine_learning::ML;

use polars::prelude::*;
use rand::Rng;


#[derive(Clone)]
pub enum DTCritetion {
  Gini,
  InformationGain
}

#[derive(Clone)]
pub struct DecisionTree<'a> {
  pub ml: &'a ML<'a>,
  pub criterion: DTCritetion,

  // pub checked_splits: HashMap<&str, >
}

impl<'a> DecisionTree <'_> {
  pub fn init(ml: &'a ML, criterion: DTCritetion) -> DecisionTree<'a> {
    return DecisionTree {ml, criterion};
  }

  pub fn train(self) {
    println!("brrrr... training");

    self.find_split();
  }

  // Pick random value from series
  pub fn pick_split_value(&'a self, s: &'a Series) -> AnyValue {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..s.len()) as usize;
    let val: AnyValue = s.get(index).ok().unwrap().clone();
    return val;
  }




  fn find_split(self){

    for col in self.ml.colnames.iter(){
      let ser: Series = self.ml.df.column(col).ok().unwrap().clone();
      let unique_vs: Series = ser.unique().ok().unwrap();
      let value_count: usize = unique_vs.len();
      let rand_v = self.pick_split_value(&unique_vs);


      self.get_indices(col, rand_v);


      if value_count == 2 {

      } else {

      }

    }

  }

  fn get_indices(&self, col: &str, val: AnyValue)  -> PolarsResult<DataFrame> {
    let mask = self.ml.df.column(col)?.is_not_null();

    self.ml.df.filter(&mask)
  }


  // fn get_indices(self, col: &str, val: AnyValue) -> Vec<bool> {
  //   let mut v: Vec<bool> = Vec::with_capacity(2000);
  //   // df.filter(&mask)
  //   // self.ml.df.filter(mask)
  //   // self.ml.df
  //   return v;

  // }



}


//   pub fn gini_impurity(&self){
//     // Gini = 1 - sum(pi)^2
//     todo!()
//   }

//   pub fn information_gain(&self){
//     // Gini = 1 - sum(pi)^2
//     todo!()
//   }

