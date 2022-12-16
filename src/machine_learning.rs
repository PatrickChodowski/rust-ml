

// Contains base trait for machine learning struct

use crate::utils::DataFrame;

pub struct MachineLearning<'a> {
  pub data: DataFrame<'a>, // Training data read from CSV
  pub target: String, // Name of target attribute
  pub index: String // Name of index attribute
}

