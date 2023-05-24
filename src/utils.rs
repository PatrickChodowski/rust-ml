
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

 




  return df;

}