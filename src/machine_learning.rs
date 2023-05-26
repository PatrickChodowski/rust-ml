
use polars::prelude::*;

pub enum MLType {
  Classification,
  Regression
}

pub struct ML <'a> {
  pub df: &'a mut DataFrame,
  pub target_name: String, // Name of target attribute
  pub index_name: String,   // Name of index attribute
  pub target: Series,
  pub index: Series,
  pub ml_type: MLType,
  pub colnames: Vec<String>
}

impl<'a> ML <'a> {
    pub fn new(df: &'a mut DataFrame, target: &str, index: &str, ml_type: MLType) -> ML<'a> {

      let ml = ML{
        target_name: target.to_string(), 
        index_name: index.to_string(), 
        target: df.drop_in_place(target).ok().unwrap(),
        index: df.drop_in_place(index).ok().unwrap(),
        ml_type,
        df,
        colnames: Vec::new()
      };
      return ml;
    }

    pub fn set_colnames(&mut self){
      let colnames = self.df.get_column_names().into_iter().map(String::from).collect();
      self.colnames = colnames;
    }
}
