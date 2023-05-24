
use polars::prelude::*;




pub fn read_csv(csv_path: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(csv_path)?
            .has_header(true)
            .finish()
}


pub fn preprocess(mut df: DataFrame) -> DataFrame {

  let cols_to_drop: [&str; 6] = ["Name", "Ticket", "Cabin", "family", "Parch", "SibSp"];
  let cols_to_ohe: Vec<&str> = vec!["Embarked", "Pclass", "Sex"];
  let cols_to_scale: Vec<&str> = vec!["Fare", "Age"];
  df = df.drop_many(&cols_to_drop);
  df = df.columns_to_dummies(cols_to_ohe, None).ok().unwrap();

  for c in cols_to_scale.iter(){
    let s: &Series = df.column(c).ok().unwrap();
    scale_zscore(s);
  }



  return df;

}



pub fn scale_zscore(s: &Series) -> &Series {

  let s_mean: f64 = s.mean().unwrap();
  // let s_stdev: f64 = s.std.unwrap();

  // s.agg_std();

  let s_std = s.std_as_series(0);

  println!("s mean: {:?}", s_mean);
  println!("s std: {:?}", s_std);

  return s;

}