
use polars::prelude::*;

// read the csv
pub fn read_csv(csv_path: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(csv_path)?
            .has_header(true)
            .finish()
}

// Main preprocessing function. 
pub fn preprocess(mut df: DataFrame) -> DataFrame {

  let cols_to_drop: [&str; 6] = ["Name", "Ticket", "Cabin", "family", "Parch", "SibSp"];
  let cols_to_ohe: Vec<&str> = vec!["Embarked", "Pclass", "Sex"];
  let cols_to_scale: Vec<&str> = vec!["Fare", "Age"];

  df = df.drop_many(&cols_to_drop);

  let _res = df.apply("Embarked", impute_forward);

  df = df.columns_to_dummies(cols_to_ohe, None).ok().unwrap();

  for c in cols_to_scale.iter(){
    let _res = df.apply(c, impute_mean);
    let _res = df.apply(c, scale_zscore);
  }

  return df;

}


// Apply z-score scaling to the series
fn scale_zscore(s: &Series) -> Series {

  let s_mean: f64 = s.mean().unwrap();
  let s_std: f64 = s.std_as_series(0).sum().unwrap();

  return s.f64()
          .unwrap()
          .into_iter()
          .map(|opt_v: Option<f64>| {
              opt_v.map(|v: f64| (v- s_mean)/s_std)
           }).collect::<Float64Chunked>().into_series();

}


// Apply z-score scaling to the series
fn impute_mean(s: &Series) -> Series {
  return s.fill_null(FillNullStrategy::Mean).ok().unwrap();
}

fn impute_forward(s: &Series) -> Series {
  return s.fill_null(FillNullStrategy::Forward(None)).ok().unwrap();
}
