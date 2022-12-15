

// Contains base trait for machine learning struct

pub struct MachineLearning<T> {
  pub data: T, // Training data read from CSV
  pub target: String, // Name of target attribute
  pub index: String // Name of index attribute
}

